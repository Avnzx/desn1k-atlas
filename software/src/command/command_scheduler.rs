use std::collections::HashSet;

use arrayvec::ArrayVec;

use super::{command::Command, subsystem::Subsystem};

const MAX_SUBSYSTEM_COUNT: usize = 4;
const MAX_RUNNING_COMMANDS: usize = 4;
const MAX_SCHEDULER_QUEUE: usize = 3;

// Command scheduler
#[derive(Default)]
pub struct CommandScheduler {
    pub disabled: bool,
    pub subsystems:
        ArrayVec<(&'static dyn Subsystem, Option<&'static dyn Command>), MAX_SUBSYSTEM_COUNT>,
    pub running_commands: ArrayVec<&'static dyn Command, MAX_RUNNING_COMMANDS>,

    pub to_schedule: ArrayVec<&'static dyn Command, MAX_SCHEDULER_QUEUE>,
}

impl CommandScheduler {
    /// Run subsystems
    /// Run scheduled commands
    /// Drop & End finished commands
    /// Try to schedule pending commands & cancel interrupted
    /// Run newly scheduled commands' initialize
    /// Try to schedule default commands
    pub fn run(&mut self) {
        if self.disabled {
            return;
        }

        for subsystem in self.subsystems.as_slice() {
            subsystem.0.periodic()
        }

        // Execute running commands
        self.running_commands.iter().for_each(|cmd| cmd.execute());

        // End running commands if they want to finish, else keep running them
        self.running_commands.retain(|cmd| {
            if cmd.is_finished() {
                cmd.end(false);
                false
            } else {
                true
            }
        });

        // Try to schedule&initialize new commands, cancel interrupted commands
        for cmd in &self.to_schedule.clone() {
            for other in &self.running_commands.clone() {
                if !cmd.is_disjoint(*other) {
                    self.cancel(*other)
                }
            }
            // All commands are interruptible so we can assume they were all successfully interrupted
            self.running_commands.push(*cmd);
            cmd.initialize();
        }
        self.to_schedule.clear();

        // TODO: redo this, it's dumb: Schedules default commands
        // We can't have subsystem overlaps, and "loose" commands (without reqs) won't be added to this
        let mut reqs = HashSet::<*const dyn Subsystem>::with_capacity(MAX_SUBSYSTEM_COUNT);
        for cmd in &self.running_commands {
            for req in cmd.get_requirements() {
                reqs.insert(*req);
            }
        }
        self.subsystems.iter().for_each(|sub| {
            if (!reqs.contains(&core::ptr::addr_of!(*sub.0))) && sub.1.is_some() {
                self.running_commands.push(sub.1.unwrap());
                sub.1.unwrap().initialize();
            }
        });
        // END Schedule default command
    }

    pub fn register_subsystem(&mut self, subsystem: &'static dyn Subsystem) {
        self.subsystems.push((subsystem, None))
    }

    pub fn unregister_subsystem(&mut self, subsystem: &'static dyn Subsystem) {
        self.subsystems.swap_remove(
            self.subsystems
                .iter()
                .position(|x| core::ptr::addr_eq(subsystem, (*x).0))
                .unwrap(),
        );
    }

    pub fn set_default_command(
        &mut self,
        subsystem: &'static dyn Subsystem,
        command: &'static dyn Command,
    ) {
        if let Some(subsys) = self
            .subsystems
            .iter_mut()
            .find(|x| core::ptr::addr_eq(subsystem, (*x).0))
        {
            subsys.1 = Some(command);
        }
    }

    pub fn unset_default_command(&mut self, subsystem: &'static dyn Subsystem) {
        if let Some(subsys) = self
            .subsystems
            .iter_mut()
            .find(|x| core::ptr::addr_eq(subsystem, (*x).0))
        {
            subsys.1 = None;
        }
    }

    // TODO: deal with commands already in the to_schedule list
    pub fn schedule(&mut self, command: &'static dyn Command) {
        self.to_schedule.push(command)
    }

    pub fn cancel(&mut self, command: &'static dyn Command) {
        self.running_commands
            .iter()
            .find(|x| core::ptr::addr_eq(command, *x));
    }
}
