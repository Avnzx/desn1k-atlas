use crate::hardware::switch_controller::SwitchController;
use std::io::Error;

use super::{command_scheduler::CommandScheduler, trigger::Trigger};

pub struct CommandSwitchController {
    pub raw: SwitchController,
}

impl CommandSwitchController {
    pub fn new(dev_path: Option<String>) -> Self {
        CommandSwitchController {
            raw: SwitchController {
                device_path: dev_path.or(Some("/dev/input/event0".into())).unwrap(),
                ..Default::default()
            },
        }
    }

    pub fn update(&mut self, _scheduler: &mut CommandScheduler) -> Result<(), Error> {
        self.raw.update()?;

        // TODO: call event loop
        Ok(())
    }

    pub fn a() -> Trigger {
        Trigger {
            condition: &|| true,
        }
    }
}
