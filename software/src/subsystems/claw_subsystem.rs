use crate::command::subsystem::Subsystem;

pub struct ClawSubsystem {}

impl ClawSubsystem {
    pub fn new() -> ClawSubsystem {
        ClawSubsystem {}
    }
}

impl Subsystem for ClawSubsystem {}
