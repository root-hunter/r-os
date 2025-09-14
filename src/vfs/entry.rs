use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum FSEntryType {
    File,
    Folder,
    Link,
}

#[derive(Serialize, Deserialize)]
pub struct FSEntry {
    pub r#type: FSEntryType,
    pub is_shared: bool,
    pub is_hidden: bool,

    pub path: String,
    pub name: String,

    pub contents: Vec<u8>,
}

impl Default for FSEntry {
    fn default() -> Self {
        Self {
            r#type: FSEntryType::File,
            is_shared: false,
            is_hidden: false,
            path: "/".into(),
            name: String::new(),
            contents: Vec::new(),
        }
    }
}