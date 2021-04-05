use std::collections::HashMap;
use std::path;
use std::fmt::Formatter;

/// WordCountError enumerates all possible errors returned by this library.
#[derive(Debug, Clone)]
pub enum KvsError {
    Unknown,
    KeyNotFound,
}

impl std::fmt::Display for KvsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            KvsError::Unknown => {
                write!(f, "Unknown Error")
            },
            KvsError::KeyNotFound => {
                write!(f, "Key not found")
            }
        }
    }
}

pub type Result<T> = std::result::Result<T,KvsError>;

pub struct KvStore {
    kv_db: HashMap<String, String>,
}


impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}


impl KvStore {
    pub fn new() -> KvStore{
        KvStore{
            kv_db: HashMap::new(),
        }
    }

    pub fn open(path: &path::Path) -> Result<KvStore> {

        return Result::Ok(KvStore::new())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        self.kv_db.insert(key,value);
        Ok(true)
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let value = self.kv_db.get(&key).cloned();
        Ok(value)
    }
    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        let val = self.kv_db.get(&key).cloned();
        if val.is_some() {
            let res = self.kv_db.remove(&key);
            Ok(res)
        }
        else { return Err(KvsError::KeyNotFound)}
    }

}

