use clap::{load_yaml, App, AppSettings, Arg, SubCommand, ArgMatches};
use kvs::{KvStore, KvsError, Result};
use std::path::PathBuf;
use std::thread::panicking;
use anyhow::Error;
use std::process::exit;
use std::env::current_dir;

fn main() -> Result<()> {
    //the YAML file is found relative to the current file, similar to modules
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    let mut kv_store = KvStore::open(current_dir().unwrap().as_path()).unwrap();

    match m.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let res  = kv_store.get(key.parse().unwrap()).unwrap();
            if res.is_none(){
                print!("Key not found");
            }
           else {
               print!("{}", res.unwrap());
           }
        },
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let value = matches.value_of("VALUE").expect("VALUE argument missing");
        },
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
           let res =  match kv_store.remove(key.parse().unwrap())
           {
               Ok(val) => {val }
               Err(KvsError::KeyNotFound) => {
                       println!("Key not found");
                       exit(1);
               },
               Err(e) => {
                   return Err(e)
               }
           };

        },
        _ => { panic!("no args")}
    }
    Ok(())
}


