use std::sync::Arc;

use crate::{core::shell::command::ShellCommandWithData, kernel::Kernel};
use async_std::sync::Mutex;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cd", about = "rOS command to change directory", version = "0.1.1")]
pub struct CdCommand {
    #[arg(required = true)]
    folder: String,
}

impl ShellCommandWithData<String> for CdCommand {
    async fn execute_data(k: &mut Kernel, cmd: &str, data: Arc<Mutex<String>>) -> String
    {
        let args = CdCommand::try_parse_from(cmd.split_whitespace());

        if let Err(e) = &args {
            return format!("{}", e);
        }

        let args = args.unwrap();

        let folder = args.folder;

        if folder.is_empty() {
            return "cd: missing operand".into();
        }

        let exists = k.fs.exists(&folder).await;
        if let Err(err) = exists {
            return format!("\nError checking existence of '{}': {:?}\n", folder, err);
        } else if !exists.unwrap() {
            return format!("\nError: folder '{}' does not exist.\n", folder);
        }

        let mut folder_mut = data.lock().await;
        *folder_mut = folder.to_string();

        format!("Changed directory to '{}'", folder)
    }
}
