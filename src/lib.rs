use std::collections::HashMap;

pub struct KvStore {
    kv_db: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore{
        KvStore{
            kv_db: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Option<&String> {
        self.kv_db.insert(key.clone(),value);
        self.kv_db.get(&key)
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.kv_db.get(&key)
    }
    pub fn remove(&self, _key: String)-> Option<String>{
        eprintln!("unimplemented");
        panic!()
    }
}

