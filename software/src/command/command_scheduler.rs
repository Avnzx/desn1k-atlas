use stack_buf::StackVec;

use super::{command::Command, subsystem::Subsystem};

// Command scheduler
#[derive(Default)]
pub struct CommandScheduler {
    pub disabled: bool,
    pub subsystems: StackVec<(&'static dyn Subsystem, Option<&'static dyn Command>), 5>,
    pub running_commands: StackVec<&'static dyn Command, 4>,

    pub to_schedule: StackVec<&'static dyn Command, 2>,
    pub to_initialize: StackVec<&'static dyn Command, 2>,
    pub to_cancel: StackVec<&'static dyn Command, 2>,
    pub to_end: StackVec<&'static dyn Command, 2>,
}

impl CommandScheduler {
      // TODO:
    pub fn run(&mut self) {
        if self.disabled {
            return;
        }

        for subsystem in self.subsystems.as_slice() {
            subsystem.0.periodic()
        }

        // Poll button loops
    }

    pub fn register_subsystem(&mut self, subsystem: &'static dyn Subsystem) {
        self.subsystems.push((subsystem, None))
    }

    pub fn unregister_subsystem(&mut self, subsystem: &'static dyn Subsystem) {
        self.subsystems.swap_remove(
            self.subsystems
                .iter()
                .position(|x| core::ptr::eq(subsystem, (*x).0))
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
            .find(|x| core::ptr::eq(subsystem, (*x).0))
        {
            subsys.1 = Some(command);
        }
    }

    pub fn unset_default_command(&mut self, subsystem: &'static dyn Subsystem) {
        if let Some(subsys) = self
            .subsystems
            .iter_mut()
            .find(|x| core::ptr::eq(subsystem, (*x).0))
        {
            subsys.1 = None;
        }
    }

    pub fn schedule(&mut self, command: &'static dyn Command) {
        self.to_schedule.push(command)
    }

    // TODO:
    pub fn cancel(&mut self) {}
}
