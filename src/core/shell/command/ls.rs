use crate::{core::shell::command::ShellCommand, kernel::Kernel, vfs::entry::FSEntryTrait};

pub struct LsCommand;


impl ShellCommand for LsCommand {
    async fn execute(k: &mut Kernel, cmd: &str) -> String {
        let mut entries = k.fs.read_folder("/").await.unwrap_or_else(|_| vec![]);

        entries.sort_by(|a, b| {
            let a_abs_path = match a.entry.to_owned() {
                crate::vfs::entry::FSEntryKind::Folder(entry) => entry.path(),
                crate::vfs::entry::FSEntryKind::File(entry) => entry.path(),
                crate::vfs::entry::FSEntryKind::Link(entry) => entry.path(),
            };

            let b_abs_path = match b.entry.to_owned() {
                crate::vfs::entry::FSEntryKind::Folder(entry) => entry.path(),
                crate::vfs::entry::FSEntryKind::File(entry) => entry.path(),
                crate::vfs::entry::FSEntryKind::Link(entry) => entry.path(),
            };

            a_abs_path.cmp(&b_abs_path)
        });

        let entries = entries
            .iter()
            .map(|entry| match &entry.entry {
                crate::vfs::entry::FSEntryKind::Folder(entry) => format!("{}", entry.path()),
                crate::vfs::entry::FSEntryKind::File(entry) => format!("{}", entry.path()),
                crate::vfs::entry::FSEntryKind::Link(entry) => format!("{}@", entry.path()),
            })
            .collect::<Vec<String>>()
            .join("\n");

        entries
    }
}
