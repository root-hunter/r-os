use serde::{Deserialize, Serialize};

pub trait FSEntryTrait {
    fn is_hidden(&self) -> bool;
    fn name(&self) -> String;
    fn created_at(&self) -> u64;
    fn modified_at(&self) -> u64;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FSEntryKind {
    File(FSFile),
    Folder(FSFolder),
    Link(FSLink),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSEntry {
    pub abs_path: String,
    pub entry: FSEntryKind,
}

impl FSEntry {
    pub fn path(&self) -> String {
        self.abs_path.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSEntryMetadata {
    pub is_hidden: bool,
    pub name: String,
    pub created_at: u64,
    pub modified_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSFolder {
    pub metadata: FSEntryMetadata,
}

impl FSEntryTrait for FSFolder {
    fn is_hidden(&self) -> bool {
        self.metadata.is_hidden
    }

    fn name(&self) -> String {
        self.metadata.name.to_string()
    }

    fn created_at(&self) -> u64 {
        self.metadata.created_at
    }

    fn modified_at(&self) -> u64 {
        self.metadata.modified_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSFile {
    metadata: FSEntryMetadata,

    pub data: Option<Vec<u8>>,
}

impl FSEntryTrait for FSFile {
    fn is_hidden(&self) -> bool {
        self.metadata.is_hidden
    }

    fn name(&self) -> String {
        self.metadata.name.to_string()
    }

    fn created_at(&self) -> u64 {
        self.metadata.created_at
    }

    fn modified_at(&self) -> u64 {
        self.metadata.modified_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSLink {
    metadata: FSEntryMetadata,
}

impl FSEntryTrait for FSLink {
    fn is_hidden(&self) -> bool {
        self.metadata.is_hidden
    }

    fn name(&self) -> String {
        self.metadata.name.to_string()
    }

    fn created_at(&self) -> u64 {
        self.metadata.created_at
    }

    fn modified_at(&self) -> u64 {
        self.metadata.modified_at
    }
}