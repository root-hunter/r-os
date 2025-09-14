use std::collections::HashMap;
use crate::vfs::storage::init_storage;

pub struct SimpleFS {
    files: HashMap<String, String>,
}

impl SimpleFS {
    pub fn new() -> Self {
        let mut s = Self { files: HashMap::new() };
        s.files.insert("readme.txt".into(), "Welcome to Rust Browser OS!\n".into());
        s
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

    pub async fn init(&mut self) {
        init_storage().await.unwrap();
    }
}
