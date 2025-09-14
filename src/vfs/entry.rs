use serde::{Deserialize, Serialize};

pub trait FSEntryTrait {
    fn is_hidden(&self) -> bool;
    fn path(&self) -> &str;
    fn name(&self) -> &str;
    fn created_at(&self) -> u64;
    fn modified_at(&self) -> u64;

    fn full_path(&self) -> String {
        format!("{}/{}", self.path(), self.name())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FSEntry {
    File(FSFile),
    Folder(FSFolder),
    Link(FSLink),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FSEntryMetadata {
    pub is_hidden: bool,
    pub path: String,
    pub name: String,
    pub created_at: u64,
    pub modified_at: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FSFolder {
    pub metadata: FSEntryMetadata,
}

impl FSEntryTrait for FSFolder {
    fn is_hidden(&self) -> bool {
        self.metadata.is_hidden
    }

    fn path(&self) -> &str {
        &self.metadata.path
    }

    fn name(&self) -> &str {
        &self.metadata.name
    }

    fn created_at(&self) -> u64 {
        self.metadata.created_at
    }

    fn modified_at(&self) -> u64 {
        self.metadata.modified_at
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FSFile {
    metadata: FSEntryMetadata,

    pub data: Option<Vec<u8>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FSLink {
    metadata: FSEntryMetadata,
}