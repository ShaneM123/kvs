use std::collections::HashMap;
use std::path;
use std::fs;
use std::fmt::Formatter;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{Write, BufReader};
use serde_json;
use serde::{Serialize,Deserialize};
use std::env::{set_current_dir, join_paths};

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
#[derive(Serialize, Deserialize, Debug)]
pub struct KvsCommand {
    command: CmdType,
    key: String,
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CmdType {
    Set,
    Get,
    Rm,
}

pub fn read_kvscommands_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<KvsCommand>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `kvscmd`.
    let kvscmd = serde_json::from_reader(reader).unwrap();

    // Return the `kvscmd`.
    Ok(kvscmd)
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
    pub fn with_old_path(path: &path::Path) -> KvStore{
        let deserialised = read_kvscommands_from_file(path).unwrap();
       // let deserialised = serde_json::from_slice::<Vec<KvsCommand>>(&x).unwrap();
        let mut kv_db:HashMap<String,String> = HashMap::new();
        for x in deserialised {
            if x.command== CmdType::Set{
                kv_db.insert(x.key,x.value.unwrap());
            }
        }
        //run the log of db commands.

        KvStore{
            kv_db,
            path: path.to_str().unwrap().parse().unwrap()
        }
    }


    pub fn open(path: &path::Path) -> Result<KvStore> {
        if path.exists(){
            println!("path exists {:?}", path);

            let the_path = Path::new(&path).join("dbcmds.txt");
             println!("The full path on exists line  {:?}", the_path);
             let  db_file = fs::File::create(the_path.clone()).unwrap();
            return Result::Ok(KvStore::with_old_path(the_path.as_path()));
        }
        fs::create_dir_all(&path);
        let the_path = Path::new(&path).join("dbcmds.txt");
        let  db_file = fs::File::create(the_path.clone()).unwrap();
        println!("The full path brand new {:?}", the_path);
        return Result::Ok(KvStore::new_with_path(the_path.as_path()))
    }


    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        self.kv_db.insert(key.clone(), value.clone());
        let set_command = KvsCommand{
            command: CmdType::Set,
            key,
            value: Option::from(value),
        };
        let formatted = serde_json::to_string(&set_command).unwrap();

        let mut file = fs::File::open(&self.path).unwrap();

        file.write(formatted.as_ref());
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

