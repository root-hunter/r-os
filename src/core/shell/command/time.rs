
use clap::Parser;

use crate::core::shell::command::ShellCommand;

#[derive(Parser, Debug)]
#[command(name = "time", about = "rOS time command", version = "0.1.0")]
pub struct TimeCommand {
}

impl ShellCommand for TimeCommand {
    async fn execute(k: &mut crate::kernel::Kernel, cmd: &str) -> String {
        let args = TimeCommand::try_parse_from(cmd.trim().split_whitespace());

        if let Err(e) = &args {
            return format!("{}", e);
        }

        let time = k.get_time();
        let timestamp = k.get_timestamp();
        
        k.print(&format!("\nSystem clock:\nTimestamp: {}\nUNIX Epoch (mills): {}\n", timestamp, time));

        "".into()
    }
}