use idb::{Query, TransactionMode};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use std::{collections::HashMap};
use wasm_bindgen::JsValue;
use crate::vfs::errors::SimpleFSError;

const REG_FOLDER: &str = r"(/?([^/\\0]+/)*[^/\\0]*)";

use crate::{
    console_log,
    vfs::{
        entry::{FSEntry, FSFolder},
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
        console_log("[vfs] storage initialized\n");
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

    pub fn is_folder_path(path: &str) -> bool {
        let re = regex::Regex::new(REG_FOLDER).unwrap();
        re.is_match(path)
    }

    pub fn is_absolute_path(path: &str) -> bool {
        path.starts_with('/')
    }

    pub async fn exists(&self, path: &str) -> Result<bool, SimpleFSError> {
        if !SimpleFS::is_folder_path(path) {
            console_log(&format!("[vfs] invalid path '{}'\n", path));
            return Err(SimpleFSError::InvalidPath);
        }

        if let Some(db) = &self.database {
            console_log(&format!("[vfs] checking if path '{}' exists\n", path));
            let transaction = db
                .transaction(&["vol_0"], TransactionMode::ReadOnly)
                .unwrap();

            let store = transaction.object_store("vol_0").unwrap();

            let key = JsValue::from_str(path);
            let req = store.get(key).unwrap();

            let result = req.await.unwrap();

            transaction.await.unwrap();

            if result.is_none() {
                console_log(&format!("[vfs] path '{}' does not exist\n", path));
                return Ok(false);
            }

            console_log(&format!("[vfs] path '{}' exists\n", path));
            return Ok(true);
        } else {
            console_log("[vfs] database not initialized\n");
            return Err(SimpleFSError::IOError);
        }
    }

    pub async fn read_folder(&self, path: &str) -> Result<Vec<FSEntry>, SimpleFSError> {
        if SimpleFS::is_folder_path(path) == false {
            console_log(&format!("[vfs] invalid path '{}'\n", path));
            return Ok(vec![]);
        }
        
        if let Some(db) = &self.database {
            console_log(&format!("[vfs] reading folder '{}'\n", path));
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
            let result = req.await.unwrap();

            console_log(&format!("[vfs] found {} entries\n", result.len()));

            if result.is_empty() {
                console_log(&format!("[vfs] folder '{}' is empty\n", path));
                return Ok(vec![]);
            }

            let entries = result
                .into_iter().map(|entry| serde_wasm_bindgen::from_value(entry).unwrap()).collect();

            transaction.await.unwrap();

            console_log(&format!("[vfs] folder '{}' read\n", path));
            Ok(entries)
        } else {
            console_log("[vfs] database not initialized\n");
            Err(SimpleFSError::IOError)
        }
    }

    pub async fn create_folder(&mut self, path: &str) -> Result<FSEntry, SimpleFSError>{
        return self.create_folder_relative("/", path).await;
    }

    pub async fn create_folder_relative(&mut self, current_folder: &str, path: &str) -> Result<FSEntry, SimpleFSError> {
        let full_path = if path.starts_with('/') {
            path.to_string()
        } else {
            if current_folder == "/" {
                format!("/{}", path)
            } else {
                format!("{}/{}", current_folder.trim_end_matches('/'), path)
            }
        };

        return self.create_folder_absolute(&full_path).await;
    }

    pub async fn create_folder_absolute(&mut self, path: &str) -> Result<FSEntry, SimpleFSError> {

        if !SimpleFS::is_folder_path(path) {
            console_log(&format!("[vfs] invalid folder name '{}'\n", path));
            return Err(SimpleFSError::InvalidPath);
        }

        if !SimpleFS::is_absolute_path(path) {
            console_log(&format!("[vfs] ivalid path '{}' it must be absolute\n", path));
            return Err(SimpleFSError::InvalidPath);
        }

        if self.exists(path).await.unwrap_or(false) {
            console_log(&format!("[vfs] folder '{}' already exists\n", path));
            return Err(SimpleFSError::AlreadyExists);
        }

        let path_parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        if path_parts.len() < 1 {
            console_log(&format!("[vfs] cannot create root folder '{}'\n", path));
            return Err(SimpleFSError::InvalidPath);
        }

        let parent_path = if path == "/" {
            "/".to_string()
        } else {
            let mut p_path = "".to_string();

            let last = path_parts.len() - 1;
            for (i, p) in path_parts.iter().enumerate() {
                if i == last {
                    continue;
                }
                
                p_path.push_str(format!("/{}", p).as_str());
            }

            if p_path.is_empty() {
                "/".to_string()
            } else {
                p_path
            }
        };

        console_log(&format!("[vfs] parent folder of '{}' is '{}'\n", path, parent_path));

        if !parent_path.is_empty() && parent_path != "/" && !self.exists(&parent_path).await.unwrap_or(false) {
            console_log(&format!("[vfs] parent folder '{}' does not exist\n", parent_path));
            return Err(SimpleFSError::ParentNotFound);
        }

        if let Some(db) = &self.database {
            console_log(&format!("[vfs] creating folder '{}'\n", path));
            
            let transaction = db
                .transaction(&["vol_0"], TransactionMode::ReadWrite)
                .unwrap();

            let name = path.rsplit('/').next().unwrap_or("");
            let store = transaction.object_store("vol_0").unwrap();

            let now = chrono::Utc::now().timestamp_millis();
            let folder = FSFolder {
                metadata: crate::vfs::entry::FSEntryMetadata {
                    name: name.into(),
                    created_at: now,
                    modified_at: now,
                    is_hidden: false,
                },
            };

            let serializer = Serializer::json_compatible();
            let entry = FSEntry {
                abs_path: path.into(),
                entry: crate::vfs::entry::FSEntryKind::Folder(folder.clone()),
            };
            self.files.insert(entry.abs_path.clone(), entry.clone());

            let id = store
                .add(&entry.serialize(&serializer).unwrap(), None)
                .unwrap()
                .await
                .unwrap();

            console_log(&format!("[vfs] folder id: {:?}\n", id));
            transaction.commit().unwrap().await.unwrap();

            console_log(&format!("[vfs] folder '{}' created\n", entry.abs_path));

            Ok(entry)
        } else {
            console_log("[vfs] database not initialized\n");

            Err(SimpleFSError::IOError)
        }
    }
}
