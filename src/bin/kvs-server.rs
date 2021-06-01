use clap::{App, Arg};
use slog::{info, o, Drain};
use slog_term;
use chrono;
use std::net::TcpListener;

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
            Ok(stream) => {
                println!("new client!");
            }
            Err(e) => {
                eprintln!("connection failed");
            }
        }
    }


}
