use std::{any::Any, sync::Arc};

use async_std::sync::Mutex;

use crate::{core::shell::Shell, kernel::Kernel};

pub mod cd;
pub mod ls;
pub mod mkdir;
pub mod touch;
pub mod exists;

pub trait ShellCommand {
    async fn execute(k: &mut Kernel, cmd: &str) -> String {
        panic!("This command must be implemented in the specific command module");
    }
}

pub trait ShellCommandWithShell {
    async fn execute(k: &mut Kernel, shell: &mut Shell, cmd: &str) -> String {
        panic!("This command must be implemented in the specific command module");
    }
}

pub trait ShellCommandWithData<T>
where
    T: AsRef<str> + Send + Sync + Any + 'static,
{
    async fn execute_data(k: &mut Kernel, cmd: &str, data: Arc<Mutex<T>>) -> String {
        panic!("This command must be implemented in the specific command module");
    }
}