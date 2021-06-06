
use clap::{App, Arg, SubCommand, ArgMatches};
use std::env::{args, Args};
use std::net::TcpStream;
use std::io::{Error as IOError, Write,ErrorKind};
use serde::Serialize;


#[derive(Debug, Clone)]
pub enum ClientError <'c> {
    Unknown,
    KeyNotFound,
    Interrupted(&'c str),
}

impl std::fmt::Display for ClientError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientError::Unknown => {
                write!(f, "Unknown Error")
            },
            ClientError::KeyNotFound => {
                write!(f, "Key not found")
            },
            //TODO: remove/fix x
            ClientError::Interrupted(x) => {
                write!(f, "stream interrupted, {}", x,)
            }
        }
    }
}

pub type Result<'a, T> = std::result::Result<T,ClientError<'a>>;

pub fn main() -> Result<'static,()> {
    let matches = App::new("kvs-client")
        .version("0.1.0")
        .author("Shane Moloney shanemoloneybusiness@gmail.com")
        .about("a db server following pingcap talent plan")
        .subcommands(vec![
            SubCommand::with_name("set")
                .args(&[Arg::with_name("KEY").required(true),
                    Arg::with_name("VALUE").required(true),
                    Arg::with_name("addr")
                        .required(true)
                        .value_name("addr")
                        .long("addr")
                        .takes_value(true),
                ])
            ,
            SubCommand::with_name("get")
                .args(&[Arg::with_name("KEY").required(true),
            Arg::with_name("addr")
                .required(true)
                .value_name("addr")
                .long("addr")
                .takes_value(true),
                ])]
        )
        .get_matches();

    let address = match matches.subcommand_matches("set"){
        None => {
            let sub_cmd = matches.subcommand_matches("get").unwrap();
            let address= sub_cmd.value_of("addr").unwrap();
            let key = sub_cmd.value_of("KEY").unwrap();
            let stream = TcpStream::connect(address);
            let mut stream = match stream {
                Ok(val) => {
                    val
                }
                Err(e) => {
                    eprintln!("couldnt connect to server tcp stream");
                    panic!();
                }
            };
            //TODO: figure out TCP Streams in rust
            //TODO: Stop duplicating the code, too verbose
            let data = GetStream{ cmd: "get", key: key };
            let  buf_json = serde_json::to_vec(&data).unwrap();
            let bytes_written=  stream.write(&buf_json).unwrap();
            if bytes_written < buf_json.len() {
                //TODO: improve error handling
                let x = &format!("Sent {}/{} bytes", bytes_written, buf_json.len());
                eprintln!("{}", x);
                panic!();
            }
        }
        Some(val) => {
            let sub_cmd = matches.subcommand_matches("set").unwrap();
            let address= sub_cmd.value_of("addr").unwrap();
            let key = sub_cmd.value_of("KEY").unwrap();
            let stream = TcpStream::connect(address);
            let mut stream = match stream {
                Ok(val) => {
                    val
                }
                Err(e) => {
                    eprintln!("couldnt connect to server tcp stream");
                    panic!();
                }
            };
            let key = sub_cmd.value_of("KEY").unwrap();
            let value = sub_cmd.value_of("VALUE").unwrap();

            let buf = SetStream{ cmd: "set", key, value };
            let  buf_json = serde_json::to_vec(&buf).unwrap();
            stream.write(&buf_json);
        }
    };
    return Ok(())
}
#[derive(Serialize)]
pub struct GetStream <'a,'b> {
    pub cmd: &'a str,
    pub key: &'b str,
}

#[derive(Serialize)]
pub struct SetStream <'a,'b, 'c> {
    pub cmd: &'a str,
    pub key: &'c str,
    pub value: &'b str,
}
