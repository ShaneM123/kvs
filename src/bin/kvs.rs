use clap::{load_yaml, App};
use kvs::KvStore;

fn main() {
    //the YAML file is found relative to the current file, similar to modules
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    let mut kv_store = KvStore::new();
    match m.value_of("set") {
        None => {}
        Some(_val) => {         eprintln!("unimplemented");
            panic!() }
    };
    match m.value_of("get") {
        None => {}
        Some(_val) => {         eprintln!("unimplemented");
            panic!() }
    };
    match m.value_of("v") {
        None => {}
        Some(_val) => {let version = env!("CARGO_PKG_VERSION");
            print!("{}",version);
        }
    }
    panic!();
    //

}
