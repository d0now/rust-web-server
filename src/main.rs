use clap::{Arg, App};
use std::error;

mod config;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {

    let matches = App::new("Rust web server.")
        .version("0.1.0")
        .author("d0now. <dolpin1402@gmail.com>")
        .about("Simple web server.")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a config.")
            .required(true)
            .takes_value(true))
        .get_matches();

    let result_code;

    if let Some(config_path) = matches.value_of("config") {
        result_code = init_server(config_path);
    } else {
        eprintln!("config not given.");
        result_code = -1;
    }

    println!("Result code: {}", result_code);
    std::process::exit(result_code)
}

fn init_server(config_path:&str) -> i32 {
    
    // Parse config file.
    let cfg = match config::parse(config_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Can't parse config: {}", err);
            return -1;
        }
    };

    // Check config file.
    if let Err(err) = cfg.check() {
        eprintln!("Invalid config: {}", err);
        return -1;
    }

    // Fork here.
    // Not implemented yet.

    // Loop here.
    // Not implemented yet.

    return 0;
}