
use clap::{App, Arg, SubCommand};
use std::env::{args, Args};
use std::net::TcpStream;
use std::io::Error;

pub fn main() {
    let matches = App::new("kvs-client")
        .version("0.1.0")
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
    let address = matches.value_of("addr").unwrap();
    let stream = TcpStream::connect(address);
    match stream {
        Ok(val) => { println!("connected to server ")}
        Err(e) => {
            eprintln!("couldnt connect to server tcp stream");
        }
    }
}
