use crate::{core::shell::command::ShellCommand, kernel::Kernel};

pub struct LsCommand;

impl ShellCommand for LsCommand {
    async fn execute(k: &mut Kernel, cmd: &str) -> String {
        let mut entries = k.fs.read_folder("/").await.unwrap_or_else(|_| vec![]);

        entries.sort_by(|a, b| {
            return a.abs_path.cmp(&b.abs_path);
        });

        let entries = entries
            .iter()
            .map(|entry| entry.path())
            .collect::<Vec<String>>()
            .join("\n");

        entries
    }
}
