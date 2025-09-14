use crate::kernel::Kernel;

pub mod ls;
pub mod mkdir;
pub mod touch;
pub mod exists;

pub trait ShellCommand {
    async fn execute(&self, k: &mut Kernel, args: Vec<&str>) -> String;
}