use idb::{Query, TransactionMode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::Serializer;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use crate::{
    log,
    vfs::{
        entry::{FSEntry, FSEntryTrait, FSFolder},
        storage::init_storage,
    },
};

pub struct SimpleFS {
    files: HashMap<String, FSEntry>,
    database: Option<idb::Database>,
}

impl SimpleFS {
    pub fn new() -> Self {
        return Self {
            files: HashMap::new(),
            database: None,
        };
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

    pub async fn read_folder(&self, path: &str) -> Result<Vec<FSEntry>, idb::Error> {
        if let Some(db) = &self.database {
            log(&format!("[vfs] reading folder '{}'\n", path));
            let transaction = db
                .transaction(&["vol_0"], TransactionMode::ReadOnly)
                .unwrap();

            let store = transaction.object_store("vol_0").unwrap();

            let key = JsValue::from_str(path);
       
            let query = Query::KeyRange(idb::KeyRange::bound(
                &JsValue::from_str(&format!("{}", path)),
                &JsValue::from_str(&format!("{}\u{FFFF}", path)),
                Some(false),
                Some(false),
            ).unwrap());

            let req = store.get_all(None, None).unwrap();
            let result = req.await?;

            log(&format!("[vfs] found {} entries\n", result.len()));

            if result.is_empty() {
                log(&format!("[vfs] folder '{}' is empty\n", path));
                return Ok(vec![]);
            }

            let entries = result
                .into_iter().map(|entry| serde_wasm_bindgen::from_value(entry).unwrap()).collect();

            transaction.await?;

            log(&format!("[vfs] folder '{}' read\n", path));
            Ok(entries)
        } else {
            log("[vfs] database not initialized\n");
            Err(idb::Error::InvalidStorageType)
        }
    }

    pub async fn create_folder(&mut self, name: &str) {
        if let Some(db) = &self.database {
            log(&format!("[vfs] creating folder '{}'\n", name));
            let transaction = db
                .transaction(&["vol_0"], TransactionMode::ReadWrite)
                .unwrap();

            let store = transaction.object_store("vol_0").unwrap();

            let now = chrono::Utc::now().timestamp_millis() as u64;
            let folder = FSFolder {
                metadata: crate::vfs::entry::FSEntryMetadata {
                    path: "".into(),
                    name: name.into(),
                    created_at: now,
                    modified_at: now,
                    is_hidden: false,
                },
            };

            let serializer = Serializer::json_compatible();
            let entry = FSEntry {
                abs_path: folder.full_path(),
                entry: crate::vfs::entry::FSEntryKind::Folder(folder.clone()),
            };
            let full_path = folder.full_path();
            self.files.insert(full_path.clone(), entry.clone());

            let key = JsValue::from_str(full_path.as_str());
            let id = store
                .add(&entry.serialize(&serializer).unwrap(), None)
                .unwrap()
                .await
                .unwrap();

            log(&format!("[vfs] folder id: {:?}\n", id));
            transaction.commit().unwrap().await.unwrap();

            log(&format!("[vfs] folder '{}' created\n", full_path));
        } else {
            log("[vfs] database not initialized\n");
        }
    }
}
