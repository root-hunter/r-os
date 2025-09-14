use std::collections::HashMap;
use idb::TransactionMode;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;

use crate::{log, vfs::{entry::{FSEntry, FSEntryTrait, FSFolder}, storage::init_storage}};

pub struct SimpleFS {
    files: HashMap<String, FSEntry>,
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

    pub fn write(&mut self, name: &str, contents: FSEntry) {
        self.files.insert(name.into(), contents);
    }

    pub fn read(&self, name: &str) -> Option<&FSEntry> {
        self.files.get(name)
    }

    pub fn list(&self) -> Vec<String> {
        let mut v: Vec<_> = self.files.keys().cloned().collect();
        v.sort();
        v
    }

    pub async fn create_folder(&mut self, name: &str) {

        if let Some(db) = &self.database {
            log(&format!("[vfs] creating folder '{}'\n", name));
            let transaction = db.transaction(&["vol_0"], TransactionMode::ReadWrite).unwrap();

            let store = transaction.object_store("vol_0").unwrap();

            let folder = FSFolder {
                metadata: crate::vfs::entry::FSEntryMetadata {
                    path: "/".into(),
                    name: name.into(),
                    created_at: 0,
                    modified_at: 0,
                    is_hidden: false,
                },
            };

            let serializer = Serializer::json_compatible();
            let entry = FSEntry {
                full_path: folder.full_path(),
                entry: crate::vfs::entry::FSEntryKind::Folder(folder.clone()),
            };
            let full_path = folder.full_path();
            self.files.insert(full_path.clone(), entry.clone());

            let key = JsValue::from_str(full_path.as_str());
            let id = store.add(&entry.serialize(&serializer).unwrap(), None)
                .unwrap().await.unwrap();

            log(&format!("[vfs] folder id: {:?}\n", id));
            transaction.commit().unwrap().await.unwrap();

            log(&format!("[vfs] folder '{}' created\n", full_path));
        } else {
            log("[vfs] database not initialized\n");
        }
    }
}
