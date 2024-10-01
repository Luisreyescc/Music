mod model;
use model::music_miner::miner;
use model::database_config::{config, database_tables, populate_db};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <directory>");
        std::process::exit(1);
    }

    let directory = &args[1];

    let extracted_data = miner::extract(directory);

    if let Err(e) = config::create_config_file() {
        eprintln!("Error creating or verifying Config.TOML: {}", e);
    }

    match config::create_database_connection() {
        Ok(connection) => {
            if let Err(e) = database_tables::create_all_tables(&connection) {
                eprintln!("Error creating tables: {}", e);
            } else {
                println!("All tables created successfully!");

                if let Err(e) = populate_db::insert_types(&connection) {
                    eprintln!("Error inserting types: {}", e);
                } else {
                    println!("Types inserted successfully!");
                }
               
                for tag_map in extracted_data {
                    if let Err(e) = populate_db::populate_database(&connection, tag_map) {
                        eprintln!("Error populating database: {}", e);
                    } else {
                        println!("Data inserted successfully!");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
        }
    }
}
