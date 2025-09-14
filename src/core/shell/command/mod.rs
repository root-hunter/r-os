use crate::kernel::Kernel;

pub mod mkdir;
pub mod touch;

pub trait ShellCommand {
    fn execute(&self, k: &mut Kernel, args: Vec<&str>) -> String;
}