use super::eventloop::EventLoop;

pub struct Trigger {
    pub condition: &'static dyn FnMut() -> bool,
}

impl Trigger {
    // TODO: Add all of them and fix this one
    pub fn on_true<const T: usize>(&mut self, _evloop: &EventLoop<T>) {}
}
