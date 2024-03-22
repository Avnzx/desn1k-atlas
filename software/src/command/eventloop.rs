use std::usize;

use arrayvec::ArrayVec;

pub struct EventLoop<const SZ: usize> {
    pub bindings: ArrayVec<&'static dyn Fn(), SZ>,
}

impl<const SZ: usize> EventLoop<SZ> {
    pub fn bind(&mut self, func: &'static dyn Fn()) {
        self.bindings.push(func)
    }

    pub fn poll(&self) {
        self.bindings.iter().for_each(|f| f())
    }
}
