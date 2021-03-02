//use clap::{load_yaml, App};
//use kvs::KvStore;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt =  Opt::from_args();
   eprintln!("unimplemented");
    panic!()

   //  //the YAML file is found relative to the current file, similar to modules
   //  let yaml = load_yaml!("cli.yml");
   //  let m = App::from(yaml).get_matches();
   // // let kv_store = KvStore::new();
   //  match m.value_of("set") {
   //      None => {}
   //      Some(_val) => {         eprintln!("unimplemented");
   //          panic!() }
   //  };
   //  match m.value_of("get") {
   //      None => {}
   //      Some(_val) => {         eprintln!("unimplemented");
   //          panic!() }
   //  };
   //  match m.value_of("v") {
   //      None => {}
   //      Some(_val) => {let version = env!("CARGO_PKG_VERSION");
   //          print!("{}",version);
   //      }
   //  }
   //  panic!();
   //  //

}



// /// A basic example
// #[derive(StructOpt)]
// struct Opt {
//     // // A flag, true if used in the command line. Note doc comment will
//     // // be used for the help message of the flag. The name of the
//     // // argument will be, by default, based on the name of the field.
//     // /// Activate debug mode
//     // #[structopt(short, long)]
//     // debug: bool,
//     //
//     // // The number of occurrences of the `v/verbose` flag
//     // /// Verbose mode (-v, -vv, -vvv, etc.)
//     // #[structopt(short, long, parse(from_occurrences))]
//     // verbose: u8,
//
//     #[structopt(flatten)]
//     Command(Command),
//     Value {
// arg2: String,
// }
// }
//
// #[derive(StructOpt)]
// enum Command {
//     Key {
//         arg1: String,
//     }
// }

#[derive(StructOpt)]
enum Opt {
    #[structopt(flatten)]
    Get(Command),

    #[structopt(flatten)]
    Set(Command),

    #[structopt(flatten)]
    Rm(Command),

}
#[derive(StructOpt)]
enum Command {
    Get {
        arg1: String,
    },
    Set {
        arg1: String,
        arg2: String,
    },
    Rm{
        arg1: String,
    },
}

