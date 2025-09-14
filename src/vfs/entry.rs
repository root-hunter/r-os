use serde::{Deserialize, Serialize};

pub trait FSEntryTrait {
    fn is_hidden(&self) -> bool;
    fn path(&self) -> &str;
    fn name(&self) -> &str;
    fn full_path(&self) -> String {
        format!("{}/{}", self.path(), self.name())
    }
}

#[derive(Serialize, Deserialize)]
pub enum FSEntry {
    File(FSFile),
    Folder(FSFolder),
    Link(FSLink),
}

#[derive(Serialize, Deserialize)]
pub struct FSFolder {
    is_hidden: bool,
    path: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FSFile {
    is_hidden: bool,
    path: String,
    name: String,

    pub data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct FSLink {
    is_hidden: bool,
    path: String,
    name: String,
}