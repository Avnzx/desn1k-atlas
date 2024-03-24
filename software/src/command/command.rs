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

impl dyn Command {
    pub fn is_disjoint(&self, other: &dyn Command) -> bool {
        !self
            .get_requirements()
            .iter()
            .find(|req| {
                other
                    .get_requirements()
                    .iter()
                    .find(|oth| core::ptr::addr_eq(**req, **oth))
                    .is_some()
            })
            .is_some()
    }
}
