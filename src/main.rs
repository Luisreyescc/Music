mod model;
use model::music_miner::miner;
use model::database_config::{config_file, database};
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

    match database::create_database_connection() {
        Ok(connection) => {
            if let Err(e) = database::create_all_tables(&connection) {
                eprintln!("Error creating tables: {}", e);
            } else {
                println!("All tables created successfully!");
            }
        }
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
        }
    }

}
