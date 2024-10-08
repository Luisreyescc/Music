mod model;
mod controller;
mod view;

use controller::controller::{cleanup_missing_songs, create_database_connection};
use std::collections::HashMap;
use model::music_miner::miner;
use model::database_config::{config, database_tables, populate_db};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <directory>");
        std::process::exit(1);
    }

    let directory = &args[1];
    run_data_pipeline(directory);
    initialize_ui();
}

fn run_data_pipeline(directory: &str) {
    let extracted_data = miner::extract(directory);

    if let Err(e) = config::create_config_file() {
        eprintln!("Error creating or verifying Config.TOML: {}", e);
    }

    match create_database_connection() {
        Ok(connection) => {
            if let Err(e) = create_and_populate_tables(&connection) {
                eprintln!("Error setting up the database: {}", e);
            } else {
                handle_missing_songs(&connection, directory);
                insert_extracted_data(&connection, extracted_data);
            }
        }
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
        }
    }
}

fn create_and_populate_tables(connection: &rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
    database_tables::create_all_tables(connection)?;
    populate_db::insert_types(connection)?;
    println!("Database setup successfully!");
    Ok(())
}

fn handle_missing_songs(connection: &rusqlite::Connection, directory: &str) {
    let directory_path = Path::new(directory);
    if let Err(e) = cleanup_missing_songs(connection, directory_path) {
        eprintln!("Error cleaning up missing songs: {}", e);
    } else {
        println!("Missing songs cleaned up successfully!");
    }
}

fn insert_extracted_data(connection: &rusqlite::Connection, extracted_data: Vec<HashMap<String, String>>) {
    for tag_map in extracted_data {
        if let Err(e) = populate_db::populate_database(connection, tag_map) {
            eprintln!("Error populating database: {}", e);
        } else {
            println!("Data inserted successfully!");
        }
    }
}

fn initialize_ui() {
    gtk::init().expect("Failed to initialize GTK.");
    view::main_ui::build_ui();
    gtk::main();
}
