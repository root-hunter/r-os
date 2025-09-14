use crate::kernel::Kernel;
use std::boxed::Box;

pub trait Process {
    fn tick(&mut self, k: &mut Kernel);
}

pub type BoxedProcess = Box<dyn Process>;