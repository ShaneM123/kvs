use clap::{load_yaml, App};

fn main() {
    //the YAML file is found relative to the current file, similar to modules
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();

    match m.value_of("arguement1") {
        //
        None => {}
        Some(_) => { println!("it worked")}
    }
    //

}
