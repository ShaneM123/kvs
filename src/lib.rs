use std::collections::HashMap;

pub struct KvStore{
    //kv_db: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore{
           // kv_db: HashMap::new(),
        }
    }
    pub fn set(&mut self, _key: String, _value: String) {
       // self.kv_db.insert(key,value)
        eprintln!("unimplemented");
        panic!()
    }

    pub fn get(&self, _key: String)-> Option<String>{
        panic!()
    }
    pub fn remove(&self, _key: String)-> Option<String>{
        panic!()
    }
}

