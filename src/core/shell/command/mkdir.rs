use crate::{console_log, core::shell::{command::{ShellCommand, ShellCommandWithShell}, Shell}, kernel::Kernel, vfs::{entry::{FSEntryTrait, FSFolder}, fs::SimpleFS}};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mkdir", about = "rOS command to create a new directory")]
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
                if let Ok(folder) = k.fs.create_folder_relative(&current_folder, &folder).await {
                    k.print(format!("mkdir: created directory '{}'", folder.path()).as_str());
                    created.push(folder);
                } else {
                    k.print(format!("mkdir: cannot create directory '{}': IO Error", folder).as_str());
                    continue;
                }
            } else {
                if let Ok(folder) = k.fs.create_folder(&folder).await {
                    k.print(format!("mkdir: created directory '{}'", folder.path()).as_str());
                    created.push(folder);
                } else {
                    k.print(format!("mkdir: cannot create directory '{}': IO Error", folder).as_str());
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