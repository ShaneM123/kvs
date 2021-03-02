use std::collections::HashMap;

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

    pub fn set(&mut self, key: String, value: String) {
        self.kv_db.insert(key,value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.kv_db.get(&key).cloned()
    }
    pub fn remove(&mut self, key: String) -> Option<String>{
        self.kv_db.remove(&key);
        self.get(key)
    }
}

