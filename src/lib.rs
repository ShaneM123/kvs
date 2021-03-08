use std::collections::HashMap;
use std::path;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum KvsError {
    #[error("unknown data store error")]
    Unknown,
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
        Ok(self.kv_db.get(&key).cloned())

    }
    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        self.kv_db.remove(&key);
        self.get(key)
    }

}

