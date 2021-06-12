
use clap::{App, Arg, SubCommand, ArgMatches};
use std::env::{args, Args};
use std::net::TcpStream;
use std::io::{Error as IOError, Write, ErrorKind, Read, BufReader, BufRead};
use serde::Serialize;
use kvs::shared::messaging::{SetStream, GetStream};
use chrono::Duration;


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
                ]),
            SubCommand::with_name("rm")
                .args(&[Arg::with_name("KEY").required(true),
                    Arg::with_name("addr")
                        .required(true)
                        .value_name("addr")
                        .long("addr")
                        .takes_value(true),
                ]),
        ]
        )
        .get_matches();
    if !( matches.is_present("set") || matches.is_present("get") || matches.is_present("rm")){
        panic!()
    }

    let address = match matches.subcommand_matches("set"){
        None => {}
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
            let bytes_written=  stream.write(&buf_json).unwrap();
            if bytes_written < buf_json.len() {
                //TODO: improve error handling
                let x = &format!("Sent {}/{} bytes", bytes_written, buf_json.len());
                eprintln!("{}", x);
                panic!();
            }
            stream.flush().unwrap();
        }
    };

    match matches.subcommand_matches("get"){
        None => {}
        Some(_) => {
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
            //TODO: deduplicate the code
            let data = SetStream{ cmd: "get", key, value: "" };
            let  buf_json = serde_json::to_vec(&data).unwrap();
            let bytes_written=  stream.write(&buf_json).unwrap();
            if bytes_written < buf_json.len() {
                //TODO: improve error handling
                let x = &format!("Sent {}/{} bytes", bytes_written, buf_json.len());
                eprintln!("{}", x);
                panic!();
            }
            let mut reader = BufReader::new(&mut stream);
            let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
            let get_result = String::from_utf8_lossy(&received);
            println!("{}",get_result);
            stream.flush().unwrap();
        }
    }

    match matches.subcommand_matches("rm"){
        None => {}
        Some(_) => {
            let sub_cmd = matches.subcommand_matches("rm").unwrap();
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
            //TODO: deduplicate the code
            let data = SetStream{ cmd: "rm", key, value: "" };
            let  buf_json = serde_json::to_vec(&data).unwrap();
            let bytes_written=  stream.write(&buf_json).unwrap();
            if bytes_written < buf_json.len() {
                //TODO: improve error handling
                let x = &format!("Sent {}/{} bytes", bytes_written, buf_json.len());
                eprintln!("{}", x);
                panic!();
            }
            let mut reader = BufReader::new(&mut stream);
            let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
            let get_result = String::from_utf8_lossy(&received);
            if get_result.len() >1 {
                eprintln!("{}", get_result);
                panic!();
            }
            stream.flush().unwrap();
        }
    }

    return Ok(())
}
