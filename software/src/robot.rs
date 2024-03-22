#[derive(Default)]
pub struct Robot {}

impl Robot {
    pub fn robot_init(&mut self) {}
    pub fn robot_periodic(&mut self) {}
    pub fn disabled_init(&mut self) {}
    pub fn disabled_periodic(&mut self) {}
    pub fn teleop_init(&mut self) {}
    pub fn teleop_periodic(&mut self) {}
}
