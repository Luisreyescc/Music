use rusqlite::{Connection, Result};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::Path; 
use gtk::prelude::*;
use gtk::ListStore;
use gtk::{Window, MessageDialog, MessageType, ButtonsType};
use crate::model::music_miner::miner; 
use crate::database_tables::create_all_tables;
use crate::populate_db::populate_database;
use crate::config::{create_database_file, create_config_dir};

/// Represents a song with its title, artist, and album.
pub struct SongDetails {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub path: String,
    pub track_number: i32,
    pub year: i32,
    pub genre: String,
}

/// Fetches a list of songs from the database with their details.
///
/// Queries the database for song details (title, artist, album, path, track number, year, and genre), 
/// handling potential `NULL` values for artist and album by using "Unknown".
///
/// # Arguments
/// * `connection` - Reference to a SQLite `Connection` for database access.
///
/// # Returns
/// * `Ok(Vec<SongDetails>)` - A vector of `SongDetails` structs.
/// * `Err(rusqlite::Error)` - If the query or mapping fails.
pub fn get_songs_from_database(connection: &Connection) -> Result<Vec<SongDetails>> {
    let mut stmt = connection.prepare(
        "SELECT rolas.title, performers.name, albums.name, rolas.path, rolas.track, rolas.year, rolas.genre
        FROM rolas
        LEFT JOIN performers ON rolas.id_performer = performers.id_performer
        LEFT JOIN albums ON rolas.id_album = albums.id_album"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(SongDetails {
            title: row.get(0)?,
            artist: row.get(1).unwrap_or_else(|_| String::from("Unknown")),
            album: row.get(2).unwrap_or_else(|_| String::from("Unknown")),
            path: row.get(3)?,
            track_number: row.get(4)?,
            year: row.get(5)?,
            genre: row.get(6)?,
        })
    })?;

    let mut songs = Vec::new();
    for song in rows {
        songs.push(song?);
    }
    Ok(songs)
}


pub fn get_song_details(title: &str) -> Result<SongDetails, Box<dyn Error>> {
    let db_path = create_database_file()?;
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT title, path, track, year, genre FROM rolas WHERE title = ?1")?;
    let mut rows = stmt.query([title])?;

    if let Some(row) = rows.next()? {
        Ok(SongDetails {
            title: row.get::<_, String>(0)?,
            artist: String::from("Unknown"),
            album: String::from("Unknown"),
            path: row.get::<_, String>(1)?,
            track_number: row.get::<_, i32>(2)?,
            year: row.get::<_, i32>(3)?,
            genre: row.get::<_, String>(4)?,
        })
    } else {
        Err("No song found with the given title".into())
    }
}

/// Populates the given `ListStore` with songs from the database.
///
/// Retrieves a list of songs from the database and populates the `ListStore` with
/// song details (title, artist, album). Clears the existing contents of the list before adding the new entries.
///
/// # Arguments
/// * `list_store` - A reference to the `ListStore` where the song data will be inserted.
///
/// # Errors
/// Prints an error to the console if the database connection fails or if songs cannot be retrieved.
pub fn populate_song_list(list_store: &ListStore) {
    let connection = match create_database_connection() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return;
        }
    };

    match get_songs_from_database(&connection) {
        Ok(songs) => {
            list_store.clear();
            
            for song in songs {
                let iter = list_store.append();
                list_store.set(&iter, &[
                    (0, &song.title), 
                    (1, &song.artist), 
                    (2, &song.album)
                ]);
            }
        },
        Err(err) => {
            eprintln!("Failed to retrieve songs from the database: {}", err);
        }
    };
}

/// Creates a connection to the SQLite database located at `~/.config/musicmanager/database.db`.
///
/// This function first ensures that the `database.db` file exists by calling `create_database_file()`.
/// Then, it attempts to create an SQLite connection to this database file. If successful,
/// the connection object is returned for further database operations.
///
/// # Returns
///
/// - `Ok(Connection)` if the connection to the database was established successfully.
/// - `Err(rusqlite::Error)` if there was an error creating the database file or opening the connection.
pub fn create_database_connection() -> Result<Connection> {
    let file_path = match create_database_file() {
        Ok(path) => path,
        Err(e) => return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e))),
    };

    let connection = Connection::open(file_path)?;
    Ok(connection)
}

pub fn remove_database_file_if_exists() -> Result<()> {
    let file_path = match create_database_file() {
        Ok(path) => path,
        Err(e) => return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e))),
    };

    if Path::new(&file_path).exists() {
        if let Err(e) = fs::remove_file(&file_path) {
            eprintln!("Error al eliminar el archivo: {}", e);
        }
    }

    Ok(())
}

/// Saves the provided music directory path to a configuration file.
///
/// Writes the music directory path to a `Config.TOML` file in the configuration directory.
/// If the file or directory does not exist, it is created.
///
/// # Arguments
/// * `directory` - The path to the music directory to save.
///
/// # Returns
/// * `Ok(())` - On success.
/// * `Err(io::Error)` - If there is an issue creating the config directory or writing to the file.
pub fn save_directory_to_config(directory: &str) -> io::Result<()> {
    let config_dir = create_config_dir()?;
    let file_path = config_dir.join("Config.TOML");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&file_path)?;

    writeln!(file, "music_directory = \"{}\"", directory)?;

    Ok(())
}

/// Displays an error dialog with a specific message.
///
/// # Parameters
/// * `window`: The main window on which the dialog is shown.
/// * `message`: The message that will be displayed in the dialog.
///
/// # Description
/// Creates a modal pop-up window (`MessageDialog`) that displays an error message
/// and an "OK" button to close the dialog. This dialog blocks interaction
/// with the main window until it is closed.
///
/// # Example
/// ```rust
/// show_error_dialog(&window, "No directory provided.");
/// ```
pub fn show_error_dialog(window: &Window, message: &str) {
    let dialog = MessageDialog::new(
        Some(window),
        gtk::DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::Ok,
        message,
    );
    dialog.run();
    dialog.close();
}

/// Ensures that the required database tables exist.
/// 
/// This function checks if all necessary tables in the database exist, and if not, it creates them.
/// It calls the internal function `create_all_tables` to handle the table creation process.
/// 
/// # Arguments
/// 
/// * `connection` - A reference to the active database connection (`&Connection`).
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the operation is successful,
///   or an error wrapped in `Box<dyn std::error::Error>` if the table creation fails.
pub fn create_tables_if_not_exist(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    create_all_tables(connection)?;
    Ok(())
}

/// Inserts song data into the database.
/// 
/// This function takes a connection to the database and a `HashMap` containing song data, 
/// and inserts that data into the corresponding table in the database.
/// It relies on the `populate_database` function to handle the actual data insertion.
/// 
/// # Arguments
/// 
/// * `connection` - A reference to the active database connection (`&Connection`).
/// * `song_data` - A `HashMap` where the keys are `String` values representing song attributes
///   (e.g., title, artist, album) and the values are their corresponding data.
/// 
/// # Returns
/// 
/// * `Result<(), rusqlite::Error>` - Returns `Ok(())` if the insertion is successful, or a 
///   `rusqlite::Error` if the operation fails.
pub fn insert_song_into_database(connection: &Connection, song_data: HashMap<String, String>) -> Result<(), rusqlite::Error> {
    populate_database(connection, song_data)
}

/// Extracts song metadata from a directory of music files.
/// 
/// This function scans the specified directory for music files and extracts metadata (such as title,
/// artist, and album) for each song. It returns a vector of `HashMap` objects, where each map
/// represents the metadata for a single song.
/// 
/// # Arguments
/// 
/// * `directory` - A string slice (`&str`) representing the path to the directory containing music files.
/// 
/// # Returns
/// 
/// * `Vec<HashMap<String, String>>` - A vector of hash maps, each representing the metadata for an extracted song.
pub fn extract_songs_from_directory(directory: &str) -> Vec<HashMap<String, String>> {
    miner::extract(directory)
}


