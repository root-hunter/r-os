#[derive(Debug)]
pub enum SimpleFSError {
    InvalidPath,
    NotFound,
    ParentNotFound,
    AlreadyExists,
    IOError,
    IndexedDBError(idb::Error),
}
impl std::fmt::Display for SimpleFSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleFSError::InvalidPath => write!(f, "Invalid path"),
            SimpleFSError::NotFound => write!(f, "Not found"),
            SimpleFSError::ParentNotFound => write!(f, "Parent folder not found"),
            SimpleFSError::AlreadyExists => write!(f, "Already exists"),
            SimpleFSError::IOError => write!(f, "IO Error"),
            SimpleFSError::IndexedDBError(e) => write!(f, "IndexedDB Error: {}", e),
        }
    }
}