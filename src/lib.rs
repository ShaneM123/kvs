use std::collections::HashMap;
use std::path;
use std::fs;
use std::fmt::Formatter;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

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
    path: String,
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
            path: "".to_string(),
        }
    }
    pub fn new_with_path(path: &path::Path) -> KvStore{
        KvStore{
            kv_db: HashMap::new(),
            path: path.to_str().unwrap().parse().unwrap()
        }
    }
    // fn open_existing_file_or_create(path: &PathBuf) -> File {
    //     fs::create_dir_all(path.parent().unwrap());
    //
    //     OpenOptions::new()
    //         .append(true)
    //         .create(true)
    //         .read(true)
    //         .open(path)
    //         .expect("could not append file")
    // }
    pub fn open(path: &path::Path) -> Result<KvStore> {
        fs::create_dir_all(&path);

        return Result::Ok(KvStore::new_with_path(path))
    }

    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        self.kv_db.insert(key.clone(),value);
        let mut file = fs::File::open(&self.path).unwrap();
        file.write(self.kv_db.get(&key).unwrap().as_bytes());
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

