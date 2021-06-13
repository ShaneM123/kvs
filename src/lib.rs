//TODO: figure out a mbetter way to mod, prob shouldnt go through lib.rs
pub mod shared;

use std::collections::HashMap;
use std::path;
use std::fs;
use std::fmt::Formatter;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{Write, BufReader, BufWriter, Read};
use serde_json;
use serde::{Serialize,Deserialize};
use std::env::{set_current_dir, join_paths};
use std::io::{prelude::*, Seek, SeekFrom};

pub trait KvsEngine{
    fn get(&self, key: String) -> Result<Option<String>> ;
    fn set(&mut self, key: String, value: String) -> Result<bool>;
    fn remove(&mut self, key: String)-> Result<()> ;
}

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
#[derive(Serialize, Deserialize, Debug,Ord, PartialOrd, Eq, PartialEq)]
pub struct KvsCommand {
    command: CmdType,
    key: String,
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum CmdType {
    Set,
    Get,
    Rm,
}

impl KvsEngine for KvStore {
    fn get(&self, key: String) -> Result<Option<String>>{
        let value = self.kv_db.get(&key).cloned();
        Ok(value)
    }

    fn set(&mut self, key: String, value: String) -> Result<bool> {
        self.kv_db.insert(key.clone(), value.clone());
        let set_command = KvsCommand {
            command: CmdType::Set,
            key,
            value: Option::from(value),
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .expect("Unable to open file");
        let mut buffy = String::new();
        file.read_to_string(&mut buffy);
        file.seek(SeekFrom::Start(0)).unwrap();

        if file.metadata().unwrap().len() <= 0
        {
            let mut x = Vec::new();
            x.push(set_command);
            let mut file = BufWriter::new(&file);
            let formatted_vec = serde_json::to_string(&x).unwrap();
            file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");
            return Ok(true);
        }
        let mut kvscmd: Vec<KvsCommand> = serde_json::from_str(&buffy).unwrap();

        kvscmd.sort();

        let mut kvscmd = kvscmd
            .into_iter()
            .filter(|x|
                if x.key.eq(&set_command.key)
                {false} else {true})
            .map(|x| x)
            .collect::<Vec<KvsCommand>>();

        kvscmd.push(set_command);

        let mut file = BufWriter::new(&file);

        let formatted_vec = serde_json::to_string(&kvscmd).unwrap();

        file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");

        Ok(true)
    }

    fn remove(&mut self, key: String)  -> Result<()> {
        let val = self.kv_db.get(&key).cloned();
        if val.is_some() {
            self.kv_db.remove_entry(&key);
        }
        else
        { return Err(KvsError::KeyNotFound) }

        let rm_command = KvsCommand {
            command: CmdType::Rm,
            key,
            value: None,
        };
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .expect("Unable to open file");
        let mut buffy = String::new();
        file.read_to_string(&mut buffy);
        file.seek(SeekFrom::Start(0)).unwrap();
        if file.metadata().unwrap().len() <= 0

        {
            let mut x = Vec::new();
            x.push(rm_command);
            let mut file = BufWriter::new(&file);
            let formatted_vec = serde_json::to_string(&x).unwrap();
            file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");
            return Ok(());
        }
        let mut kvscmd: Vec<KvsCommand> = serde_json::from_str(&buffy).unwrap();
        kvscmd.push(rm_command);

        let mut file = BufWriter::new(&file);

        let formatted_vec = serde_json::to_string(&kvscmd).unwrap();

        file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");
        Ok(())
    }
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            kv_db: HashMap::new(),
            path: "".to_string(),
        }
    }
    pub fn new_with_path(path: &path::Path) -> KvStore {
        KvStore {
            kv_db: HashMap::new(),
            path: path.to_str().unwrap().parse().unwrap()
        }
    }
    pub fn with_old_path(path: &path::Path) -> KvStore {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("Unable to open file");
        file.seek(SeekFrom::Start(0)).unwrap();

        let reader = BufReader::new(&file);

        let mut deserialised: Vec<KvsCommand> = serde_json::from_reader(reader).unwrap();

        let mut kv_db: HashMap<String, String> = HashMap::new();
        for x in deserialised {
            if x.command == CmdType::Set {
                kv_db.insert(x.key, x.value.unwrap());
            } else if x.command == CmdType::Rm {
                kv_db.remove_entry(&x.key);
            }
        }
        //run the log of db commands.
        KvStore {
            kv_db,
            path: path.to_str().unwrap().parse().unwrap()
        }
    }


    pub fn open(path: &path::Path) -> Result<KvStore> {
        if path.exists() {
            let the_path = Path::new(&path).join("dbcmds.txt");
            return if the_path.is_file() {
                Result::Ok(KvStore::with_old_path(the_path.as_path()))
            } else {
                fs::File::create(the_path.clone()).unwrap();
                Result::Ok(KvStore::new_with_path(the_path.as_path()))
            }
        }
        fs::create_dir_all(&path);
        let the_path = Path::new(&path).join("dbcmds.txt");
        let db_file = fs::File::create(the_path.clone()).unwrap();
        //  println!("The full path brand new {:?}", the_path);
        return Result::Ok(KvStore::new_with_path(the_path.as_path()))
    }
/*

    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        self.kv_db.insert(key.clone(), value.clone());
        let set_command = KvsCommand {
            command: CmdType::Set,
            key,
            value: Option::from(value),
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .expect("Unable to open file");
        let mut buffy = String::new();
        file.read_to_string(&mut buffy);
        file.seek(SeekFrom::Start(0)).unwrap();

        if file.metadata().unwrap().len() <= 0
        {
            let mut x = Vec::new();
            x.push(set_command);
            let mut file = BufWriter::new(&file);
            let formatted_vec = serde_json::to_string(&x).unwrap();
            file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");
            return Ok(true);
        }
        let mut kvscmd: Vec<KvsCommand> = serde_json::from_str(&buffy).unwrap();

        kvscmd.sort();

        let mut kvscmd = kvscmd
            .into_iter()
            .filter(|x|
                if x.key.eq(&set_command.key)
                {false} else {true})
            .map(|x| x)
        .collect::<Vec<KvsCommand>>();

        kvscmd.push(set_command);

        let mut file = BufWriter::new(&file);

        let formatted_vec = serde_json::to_string(&kvscmd).unwrap();

        file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");

        Ok(true)
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let value = self.kv_db.get(&key).cloned();
        Ok(value)
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        let val = self.kv_db.get(&key).cloned();
        if val.is_some() {
            self.kv_db.remove_entry(&key);
        }
        else
        { return Err(KvsError::KeyNotFound) }

        let rm_command = KvsCommand {
            command: CmdType::Rm,
            key,
            value: None,
        };
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .expect("Unable to open file");
        let mut buffy = String::new();
        file.read_to_string(&mut buffy);
        file.seek(SeekFrom::Start(0)).unwrap();
        if file.metadata().unwrap().len() <= 0

        {
            let mut x = Vec::new();
            x.push(rm_command);
            let mut file = BufWriter::new(&file);
            let formatted_vec = serde_json::to_string(&x).unwrap();
            file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");
            return Ok(());
        }
        let mut kvscmd: Vec<KvsCommand> = serde_json::from_str(&buffy).unwrap();
        kvscmd.push(rm_command);

        let mut file = BufWriter::new(&file);

        let formatted_vec = serde_json::to_string(&kvscmd).unwrap();

        file.write_all(formatted_vec.as_bytes()).expect("Unable to write data");
        Ok(())
    }*/

}

