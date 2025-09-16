
use clap::Parser;

use crate::{core::{shell::command::ShellCommand}};

#[derive(Parser, Debug)]
#[command(name = "top", about = "rOS process manager command", version = "0.1.0")]
pub struct TopCommand {
}

impl ShellCommand for TopCommand {
    async fn execute(k: &mut crate::kernel::Kernel, cmd: &str) -> String {
        let args = TopCommand::try_parse_from(cmd.trim().split_whitespace());

        if let Err(e) = &args {
            return format!("{}", e);
        }
        
        let mut output = format!("\n{:<30} {:<30}\n", "Process Name", "PID");

        for p in k.processes.values() {
            let pid = p.pid();
            let pname = p.name();

            output.push_str(&format!("{:<30} {:<30}\n", pname, pid));
        }
        
        k.print(&output);

        "".into()
    }
}