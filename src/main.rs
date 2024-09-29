mod model;
use model::music_miner::miner;
use model::database::config_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
   
    if args.len() < 2 {
        eprintln!("Usage: cargo run <directory>");
        std::process::exit(1);
    }

    let directory = &args[1];
    miner::extract(directory);
    
    if let Err(e) = config_file::create_config_file() {
        eprintln!("Error creating or verifying Config.TOML: {}", e);
    }


}
