use crate::{core::shell::command::ShellCommand, kernel::Kernel};

pub struct ExistsCommand;

impl ShellCommand for ExistsCommand {
    async fn execute(&self, k: &mut Kernel, args: Vec<&str>) -> String {
        if args.is_empty() {
            return "exists: missing operand".into();
        }
        let dir_name = args[0];

        let exists = k.fs.exists(dir_name).await;

        if exists {
            format!("Entry '{}' exists.", dir_name)
        } else {
            format!("Entry '{}' does not exist.", dir_name)
        }
    }
}
