use crate::{core::shell::command::ShellCommand, kernel::Kernel, vfs::entry::FSEntryTrait};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ls", about = "rOS command ", version = "0.1.1")]
pub struct LsCommand {
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

impl ShellCommand for LsCommand {
    async fn execute(k: &mut Kernel, cmd: &str) -> String {
        let args = LsCommand::try_parse_from(cmd.trim().split_whitespace());

        if let Err(e) = &args {
            return format!("{}", e);
        }

        let args = args.unwrap();

        let mut entries = k.fs.read_folder("/").await.unwrap_or_else(|_| vec![]);

        entries.sort_by(|a, b| {
            return a.abs_path.cmp(&b.abs_path);
        });

        k.print(format!("\n{:<30} {:<20} {:<20}", "Name", "Created At", "Modified At").as_str());

        let entries = entries
            .iter()
            .map(|entry| {
                let abs_path = entry.abs_path.clone();
                if args.all {
                    format!("{}", abs_path)
                } else {
                    let entry = entry.entry.clone();
                    let created_at = chrono::DateTime::from_timestamp_millis(entry.created_at())
                        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let modified_at = chrono::DateTime::from_timestamp_millis(entry.modified_at())
                        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let output = "".to_string();

                    format!("{:<30} {:<20} {:<20}", abs_path, created_at, modified_at)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        entries
    }
}
