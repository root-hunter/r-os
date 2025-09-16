use crate::{core::shell::command::ShellCommand, kernel::Kernel};

pub struct ExistsCommand;

impl ShellCommand for ExistsCommand {
    async fn execute(k: &mut Kernel, cmd: &str) -> String {
        let args: Vec<&str> = cmd.split_whitespace().skip(1).collect();
        
        if args.is_empty() {
            return "exists: missing operand".into();
        }
        let dir_name = args[0];

        let exists = k.fs.exists(dir_name).await;

        if let Err(err) = exists {
            return format!("exists: error checking existence of '{}': {:?}", dir_name, err);
        } else {
            let exists = exists.unwrap();
            if exists {
                return format!("Entry '{}' exists.", dir_name)
            } else {
                return format!("Entry '{}' does not exist.", dir_name);
            }
        }
    }
}
