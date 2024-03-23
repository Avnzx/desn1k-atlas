use crate::command::subsystem::Subsystem;

pub struct DriveSubsystem {}

impl DriveSubsystem {
    pub fn new() -> DriveSubsystem {
        DriveSubsystem {}
    }
}

impl Subsystem for DriveSubsystem {
    fn periodic(&self) {
      println!("periodic is executing, wowza!")
    }
}
