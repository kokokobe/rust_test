use std::{env, process};
use briliang_exercise::Config;



fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = briliang_exercise::run(config) {
        eprintln!("Application  error :{}", e);
        process::exit(1);
    };

}