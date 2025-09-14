use crate::kernel::{Kernel, Message};
use std::{any::Any, boxed::Box};

pub trait Process: Any {
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
    fn pid(&self) -> usize;
    fn set_pid(&mut self, pid: usize);
    fn tick(&mut self, k: &mut Kernel);
    fn on_message(&mut self, k: &mut Kernel, msg: Message) {
        // default: ignora
    }
}

pub type BoxedProcess = Box<dyn Process>;