
use clap::{App, Arg, SubCommand};
use std::env::{args, Args};

pub fn main() {
    let matches = App::new("kvs-client")
        .version("1")
        .author("Shane Moloney shanemoloneybusiness@gmail.com")
        .about("a db server following pingcap talent plan")
        .subcommands(vec![
            SubCommand::with_name("set")
                .args(&[Arg::with_name("KEY").required(true),Arg::with_name("VALUE").required(true),])
            ,
            SubCommand::with_name("get")
                .arg(Arg::with_name("KEY").required(true)),
                ]
        )
        .arg(
            Arg::with_name("addr")
                .required(true)
                .value_name("addr")
                .long("addr")
                .takes_value(true)
        )
        .get_matches();

    // .arg(Arg::with_name("Engine")
    //     .long("engine")
    //     .value_name("ENGINE-NAME")
    //     .help("ENGINE-NAME must be either kvs or sled")
    //     .takes_value(true)
    // )
    // .arg(Arg::with_name("Version")
    //     .short("V")
    //     .long("version")
    //     .help("prints the version")
    // )
}
