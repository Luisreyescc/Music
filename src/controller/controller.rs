use rusqlite::{params, Connection, Result};
use std::collections::HashSet;
use std::path::Path; 
use crate::model::music_miner::miner; 
use crate::config::create_database_file;

/// Retrieves song titles from the database and returns them as a set of strings.
///
/// # Arguments
///
/// * `connection` - A reference to the SQLite database connection.
///
/// # Returns
///
/// Returns a `Result` containing a `HashSet<String>` with the song titles in the database.
/// If an error occurs during the query, it returns an error.
fn get_song_titles_from_database(connection: &Connection) -> Result<HashSet<String>> {
    let mut stmt = connection.prepare("SELECT title FROM rolas")?;
    let rows = stmt.query_map([], |row| {
        let title: String = row.get(0)?;
        Ok(title)
    })?;
    
    let mut db_songs = HashSet::new();
    for row in rows {
        db_songs.insert(row?);
    }
    Ok(db_songs)
}

/// Cleans up songs from the database that are no longer present in the specified music directory.
///
/// This function compares the song titles in the database with the file titles in
/// the directory and deletes from the database any titles that are no longer present in the directory.
///
/// # Arguments
///
/// * `connection` - A reference to the SQLite database connection.
/// * `directory_path` - The path to the directory containing the music files.
///
/// # Returns
///
/// Returns a `Result` indicating whether the operation was successful or if an error occurred.
pub fn cleanup_missing_songs(connection: &Connection, directory_path: &Path) -> Result<()> {
    let db_songs = get_song_titles_from_database(connection)?;
    
    let mut dir_songs = HashSet::new();

    for tag_map in miner::extract(directory_path.to_str().unwrap()) {
        if let Some(title) = tag_map.get("Title") {
            dir_songs.insert(title.clone());
        }
    }
    
    for title in db_songs.difference(&dir_songs) {
        connection.execute("DELETE FROM rolas WHERE title = ?1", params![title])?;
    }
    
    Ok(())
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
