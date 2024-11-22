mod config;

use std::env;
use std::path::Path;
use config::config::{ read_config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = &args[1];

    let config  = read_config(Path::new(&config_path));

    std::print!("{:?}", config);
}
