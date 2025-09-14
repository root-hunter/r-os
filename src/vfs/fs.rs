use std::collections::HashMap;
use crate::{log, vfs::storage::init_storage};

pub struct SimpleFS {
    files: HashMap<String, String>,
    database: Option<idb::Database>,
}

impl SimpleFS {
    pub fn new() -> Self {
        return Self { files: HashMap::new(), database: None };
    }

    pub async fn init(&mut self) {
        let database = init_storage().await.unwrap();
        self.database = Some(database);
        log("[vfs] storage initialized\n");
    }

    pub fn write(&mut self, name: &str, contents: &str) {
        self.files.insert(name.into(), contents.into());
    }

    pub fn read(&self, name: &str) -> Option<&String> {
        self.files.get(name)
    }

    pub fn list(&self) -> Vec<String> {
        let mut v: Vec<_> = self.files.keys().cloned().collect();
        v.sort();
        v
    }
}
