use crate::{core::shell::command::ShellCommand, kernel::Kernel, vfs::entry::FSEntryTrait};

pub struct LsCommand;

impl ShellCommand for LsCommand {
    async fn execute(&self, k: &mut Kernel, args: Vec<&str>) -> String {
        let mut entries = k.fs.read_folder("/").await.unwrap_or_else(|_| vec![]);

        entries.sort_by(|a, b| {
            let a_abs_path = match a.entry.to_owned() {
                crate::vfs::entry::FSEntryKind::Folder(entry) => entry.full_path(),
                crate::vfs::entry::FSEntryKind::File(entry) => entry.full_path(),
                crate::vfs::entry::FSEntryKind::Link(entry) => entry.full_path(),
            };

            let b_abs_path = match b.entry.to_owned() {
                crate::vfs::entry::FSEntryKind::Folder(entry) => entry.full_path(),
                crate::vfs::entry::FSEntryKind::File(entry) => entry.full_path(),
                crate::vfs::entry::FSEntryKind::Link(entry) => entry.full_path(),
            };

            a_abs_path.cmp(&b_abs_path)
        });

        let entries = entries
            .iter()
            .map(|entry| match &entry.entry {
                crate::vfs::entry::FSEntryKind::Folder(entry) => format!("{}/", entry.full_path()),
                crate::vfs::entry::FSEntryKind::File(entry) => format!("{}", entry.full_path()),
                crate::vfs::entry::FSEntryKind::Link(entry) => format!("{}@", entry.full_path()),
            })
            .collect::<Vec<String>>()
            .join("\n");

        entries
    }
}
