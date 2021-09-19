use clap::{Arg, App};

use std::error;

mod config;
mod process;
mod request;

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

    if let Some(config_path) = matches.value_of("config") {
        let cfg = match init_server(config_path) {
            Ok(cfg) => cfg,
            Err(err) => {
                return Err(format!("Failed to initialize server: {}", err).into());
            }
        };
        serve(cfg)
    } else {
        eprintln!("Somehow reasons, config not passed.");
        Ok(())
    }
}

fn init_server(config_path:&str) -> Result<config::Config> {
    
    // Parse config file.
    let cfg = config::parse(config_path)?;

    // Check config file.
    if let Err(err) = cfg.check() {
        return Err(format!("Invalid config: {}", err).into());
    }

    // Fork here.
    // Not implemented yet.

    Ok(cfg)
}

fn serve(cfg: config::Config) -> Result<()> {
    let listener = process::bind_server(&cfg)?;
    process::process(cfg, listener)
}