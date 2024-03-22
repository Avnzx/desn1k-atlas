use crate::hardware::switch_controller::SwitchController;

pub struct CommandSwitchController {
    pub raw: SwitchController,
}

impl CommandSwitchController {
    pub fn new(dev_path: String) -> Self {
        CommandSwitchController {
            raw: SwitchController {
                device_path: dev_path,
                ..Default::default()
            },
        }
    }
}

impl Default for CommandSwitchController {
    fn default() -> Self {
        CommandSwitchController::new("/dev/input/event0".into())
    }
}
