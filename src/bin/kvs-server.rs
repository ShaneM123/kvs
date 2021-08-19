use clap::{App, Arg};
use slog::{info, o, Drain};
use slog_term;
use chrono;
use std::net::TcpListener;
use std::io::{Read, BufReader, BufRead, Error, Write};
use kvs::shared::messaging::SetStream;
use std::io;
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use std::env::temp_dir;
use std::borrow::Borrow;
use std::ops::Deref;

pub enum EngineType {
    KvsEng(KvStore),
    SledEng(SledKvsEngine),
}
//TODO: improve with less gammy solution
pub fn engine_func(typ: &str)-> Result<EngineType, Error>{
    return if typ.contains("kvs") {
        Ok(EngineType::KvsEng(KvStore::open(temp_dir().as_path()).unwrap()))
    } else {
        Ok(EngineType::SledEng(SledKvsEngine::open(&temp_dir().as_path()).unwrap()))
    };
}
pub fn main(){
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_logger = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    info!(root_logger, "Application started";
        "started_at" => format!("{}", chrono::Utc::now()));


    let matches = App::new("kvs-server")
        .version("0.1.0")
        .author("Shane Moloney shanemoloneybusiness@gmail.com")
        .about("a db server following pingcap talent plan")
        .arg(Arg::with_name("address")
            .long("addr")
            .value_name("IP-PORT")
            .help("set the ip port for kvs")
            .takes_value(true)
        )
        .arg(Arg::with_name("Engine")
            .long("engine")
            .value_name("ENGINE-NAME")
            .help("ENGINE-NAME must be either kvs or sled")
            .takes_value(true)
        )
        .get_matches();
    let ip_addr = matches.value_of("address").unwrap();
    let engine = matches.value_of("Engine").unwrap();

   info!(root_logger, "App Ended"; "version" => env!("CARGO_PKG_VERSION"), "Engine" => engine, "address" => ip_addr );

    let listener = TcpListener::bind(ip_addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
             //   println!("new client!");
                let temp_dir = std::path::Path::new("tmp/foo2.txt").file_name().unwrap();
                let mut store = match engine_func(engine).unwrap() {
                    EngineType::KvsEng(mut store) => {
                        //TODO: separate functions and refactor

                        let mut reader = BufReader::new(&mut stream);
                        let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
                        let deserialized = serde_json::from_slice::<SetStream>(&received).unwrap();
                        //    println!("DESERIALIZED:  {:?}", deserialized);
                        let x =  reader.consume(received.len());

                        if deserialized.cmd.eq("set") {
                            let x = store.set(deserialized.key.into(), deserialized.value.into()).unwrap();
                        }
                        if deserialized.cmd.eq("get") {
                            let data = match store.get(deserialized.key.into()).unwrap()
                            {
                                None => {"Key not found".to_owned()}
                                Some(val) => {val}
                            };
                            let  buf_json = data.as_ref();
                            let bytes_written=  stream.write(buf_json).unwrap();
                            if bytes_written < buf_json.len() {
                                //TODO: improve error handling
                                let x = &format!("Sent {}/{} bytes", bytes_written, buf_json.len());
                                eprintln!("{}", x);
                                panic!();
                            }
                        }

                        if deserialized.cmd.eq("rm") {
                            let x = match store.remove(deserialized.key.into())
                            {
                                Err(..) => {
                                    let  buf_json = "Key not found".as_ref();
                                    let bytes_written=  stream.write(buf_json).unwrap();
                                    if bytes_written < buf_json.len() {
                                        //TODO: improve error handling
                                        let x = &format!("Sent {}/{} bytes", bytes_written, buf_json.len());
                                        eprintln!("{}", x);
                                        panic!();
                                    }
                                }
                                Ok(()) => {}
                            };

                        }

                    }
                    EngineType::SledEng(store) => {unimplemented!()}
                };

            }
            Err(e) => {
                eprintln!("connection failed");
                panic!();
            }
        }
    }


}
