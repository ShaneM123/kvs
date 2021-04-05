use clap::{load_yaml, App, AppSettings, Arg, SubCommand, ArgMatches};
use kvs::{KvStore, KvsError, Result};
use std::path::PathBuf;
use std::thread::panicking;
use anyhow::Error;
use std::process::exit;

fn main() -> Result<()> {
    //the YAML file is found relative to the current file, similar to modules
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    let mut kv_store = KvStore::new();

    match m.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let res  = kv_store.get(key.parse().unwrap()).unwrap();
            if res.is_none(){
                print!("Key not found");
            }
            Ok(())
        },
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let value = matches.value_of("VALUE").expect("VALUE argument missing");
            Ok(())

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

            if res.is_none() {

            };
            Ok(())
        },


        _ => { panic!("no args")}
    }
    // match m.value_of("set") {
    //     None => {}
    //     Some(_val) => {
    //     }
    // };
    // match m.value_of("get") {
    //     None => { panic!("no val")}
    //     Some(val) => {
    //        match kv_store.get(val.parse().unwrap()).unwrap() {
    //            None => print!("key not found"),
    //            Some(val) => {}
    //        }
    //     }
    // };
    // match m.value_of("v") {
    //     None => {}
    //     Some(_val) => {let version = env!("CARGO_PKG_VERSION");
    //         print!("{}",version);
    //     }
    // }
    //

    }


