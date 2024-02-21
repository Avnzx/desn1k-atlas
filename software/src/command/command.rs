use super::subsystem::Subsystem;

pub trait Command {
    // The primary 4 functions
    fn initialize(&self);
    fn execute(&self);
    fn end(&self, interrupted: bool);
    fn is_finished(&self) -> bool;

    // necessary for scheduling
    fn get_requirements(&self) -> &[&'static dyn Subsystem];
}
