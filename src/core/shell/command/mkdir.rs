use crate::{console_log, core::shell::{command::ShellCommandWithShell, Shell}, kernel::Kernel, vfs::fs::SimpleFS};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mkdir", about = "rOS command to create a new directory", version = "0.1.1")]
pub struct MkDirCommand {
    #[arg(required = true)]
    folders: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

impl ShellCommandWithShell for MkDirCommand {
    async fn execute(k: &mut Kernel, shell: &mut Shell, cmd: &str) -> String {
        let args = MkDirCommand::try_parse_from(cmd.trim().split_whitespace());

        if let Err(e) = &args {
            return format!("{}", e);
        }

        let args = args.unwrap();
        let folders = args.folders;

        let mut created = Vec::new();

        for folder in folders {
            console_log(&format!("mkdir: creating directory '{}'", folder));

            let current_folder = shell.folder.lock().await.clone();

            if !SimpleFS::is_folder_path(&folder) {
                k.print(format!("mkdir: cannot create directory '{}': Not valid folder path", folder).as_str());
                continue;
            } else if k.fs.exists(&folder).await.unwrap_or(false) {
                k.print(format!("mkdir: cannot create directory '{}': File exists", folder).as_str());
                continue;
            }

            if !SimpleFS::is_absolute_path(&folder) {
                let result = k.fs.create_folder_relative(&current_folder, &folder).await;

                if let Ok(folder) = result {
                    k.print(format!("mkdir: created directory '{}'", folder.path()).as_str());
                    created.push(folder);
                } else {
                    k.print(format!("mkdir: cannot create directory '{}': {}", folder, result.unwrap_err()).as_str());
                    continue;
                }
            } else {
                let result = k.fs.create_folder(&folder).await;

                if let Ok(folder) = result {
                    k.print(format!("mkdir: created directory '{}'", folder.path()).as_str());
                    created.push(folder);
                } else {
                    k.print(format!("mkdir: cannot create directory '{}': {}", folder, result.unwrap_err()).as_str());
                    continue;
                }
            }

        }

        return if created.is_empty() {
            "".into()
        } else if args.verbose {
            format!("Created {} directories", created.len())
        } else {
            created.iter().map(|f| f.path()).collect::<Vec<String>>().join("\n")
        };
    }
}