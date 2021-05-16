use clap::{App, Arg};
use slog::{info, o, Drain};
use slog_term;
use chrono;

pub fn main(){
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_logger = slog::Logger::root(drain, o!());

    info!(root_logger, "Application started";
        "started_at" => format!("{}", chrono::Utc::now()));

    let matches = App::new("kvs-server")
        .version("1")
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
        .arg(Arg::with_name("Version")
            .short("V")
            .long("version")
            .help("prints the version")
        )
        .get_matches();

}
