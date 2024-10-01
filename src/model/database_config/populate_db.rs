extern crate rusqlite;
use chrono::Datelike;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use rusqlite::OptionalExtension;

/// Inserts default types into the "types" table.
/// These types represent different kinds of entities, such as "Person", "Group", or "Unknown".
/// It executes three insert statements, one for each type.
pub fn insert_types(connection: &Connection) -> Result<()> {
    connection.execute("INSERT INTO types VALUES(?1,?2)", (0, "Person"),)?;
    connection.execute("INSERT INTO types VALUES(?1,?2)", (1, "Group"),)?;
    connection.execute("INSERT INTO types VALUES(?1,?2)", (2, "Unknown"),)?;
    Ok(())
}

/// Finds or inserts a performer (artist) in the "performers" table.
/// It checks if the performer already exists using their name. If found, it returns their ID.
/// If not found, it inserts a new performer and returns the newly created ID.
fn insert_or_get_performer(connection: &Connection, artist: &str) -> Result<i64> {
    let mut stmt = connection.prepare("SELECT id_performer FROM performers WHERE name = ?1")?;
    let performer_id: Option<i64> = stmt.query_row(params![artist], |row| row.get(0)).optional()?;

    match performer_id {
        Some(id) => Ok(id), 
        None => {
            connection.execute("INSERT INTO performers (name) VALUES (?1)", params![artist])?;
            let new_id = connection.last_insert_rowid();
            Ok(new_id)
        }
    }
}

/// Finds or inserts an album in the "albums" table.
/// It checks if the album already exists by matching its name and release year.
/// If found, it returns the album's ID. If not, it inserts a new album and returns the new ID.
fn insert_or_get_album(connection: &Connection, album: &str, year: i32) -> Result<i64> {
    let mut stmt = connection.prepare("SELECT id_album FROM albums WHERE name = ?1 AND year =?2")?;
    let album_id: Option<i64> = stmt.query_row(params![album,year], |row| row.get(0)).optional()?;

    match album_id {
        Some(id) => Ok(id), 
        None => {
            connection.execute("INSERT INTO albums (name, year) VALUES (?1, ?2)", params![album, year])?;
            let new_id = connection.last_insert_rowid();
            Ok(new_id)
        }
    }
}

/// Inserts a new track (song) into the "rolas" table.
/// This function adds a new track with the associated performer and album IDs, title, track number, year, and genre.
fn insert_rola(
    connection: &Connection,
    performer_id: i64,
    album_id: i64,
    title: &str,
    track: i32,
    year: i32,
    genre: &str
) -> Result<()> {
    connection.execute(
        "INSERT INTO rolas (id_performer, id_album, title, track, year, genre, path) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)",
        params![performer_id, album_id, title, track, year, genre]
    )?;
    Ok(())
}

/// Populates the database with a new song (track) based on the provided tag map.
/// The tag map contains metadata such as the artist's name, album, title, track number, year, and genre.
/// It first finds or inserts the performer and album, then inserts the new track (song).
pub fn populate_database(connection: &Connection, tag_map: HashMap<String, String>) -> Result<()> {
    let artist = tag_map.get("Artist").unwrap();
    let title = tag_map.get("Title").unwrap();
    let album = tag_map.get("Album").unwrap();
    let year = tag_map.get("Year").unwrap().parse::<i32>().unwrap_or(chrono::Utc::now().year());
    let genre = tag_map.get("Genre").unwrap();
    let track_number = tag_map.get("Track Number").unwrap().parse::<i32>().unwrap_or(0);

    let performer_id = insert_or_get_performer(connection, artist)?;

    let album_id = insert_or_get_album(connection, album, year)?;

    insert_rola(connection, performer_id, album_id, title, track_number, year, genre)?;

    Ok(())
}
