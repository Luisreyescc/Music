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

/// Checks if the performer metadata has changed (e.g., name).
/// If there are changes, it returns `true` to indicate that an update is needed.
fn performer_needs_update(connection: &Connection, performer_id: i64, artist: &str) -> Result<bool> {
    let mut stmt = connection.prepare("SELECT name FROM performers WHERE id_performer = ?1")?;
    let result: Option<String> = stmt.query_row(params![performer_id], |row| row.get(0)).optional()?;

    if let Some(db_artist) = result {
        Ok(artist != db_artist)
    } else {
        Ok(false)
    }
}

/// Finds or inserts a performer (artist) in the "performers" table.
/// It checks if the performer already exists using their name. If found, it returns their ID.
/// If not found, it inserts a new performer and returns the newly created ID.
fn insert_or_update_performer(connection: &Connection, artist: &str) -> Result<i64> {
    let mut stmt = connection.prepare("SELECT id_performer FROM performers WHERE name = ?1")?;
    let performer_id: Option<i64> = stmt.query_row(params![artist], |row| row.get(0)).optional()?;

    match performer_id {
        Some(id) => {
            if performer_needs_update(connection, id, artist)? {
                connection.execute(
                    "UPDATE performers SET name = ?1 WHERE id_performer = ?2",
                    params![artist, id]
                )?;
                println!("Updated performer: {}", artist);
            }
            Ok(id)
        },
        None => {
            connection.execute(
                "INSERT INTO performers (name) VALUES (?1)",
                params![artist]
            )?;
            let new_id = connection.last_insert_rowid();
            println!("Inserted new performer: {}", artist);
            Ok(new_id)
        }
    }
}

/// Checks if the album metadata has changed (e.g., name, year).
/// If there are changes, it returns `true` to indicate that an update is needed.
fn album_needs_update(connection: &Connection, album_id: i64, album: &str, year: i32, path: &str) -> Result<bool> {
    let mut stmt = connection.prepare("SELECT name, year, path FROM albums WHERE id_album = ?1")?;
    let result: Option<(String, i32, String)> = stmt.query_row(params![album_id], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    }).optional()?;

    if let Some((db_album, db_year, db_path)) = result {
        Ok(album != db_album || year != db_year || path != db_path)
    } else {
        Ok(false)
    }
}

/// Finds or inserts an album in the "albums" table.
/// It checks if the album already exists by matching its name and release year.
/// If found, it returns the album's ID. If not, it inserts a new album and returns the new ID.
fn insert_or_update_album(connection: &Connection, album: &str, year: i32, path: &str) -> Result<i64> {
    let mut stmt = connection.prepare("SELECT id_album FROM albums WHERE path = ?1 AND name = ?2 AND year = ?3")?;
    let album_id: Option<i64> = stmt.query_row(params![path, album, year], |row| row.get(0)).optional()?;

    match album_id {
        Some(id) => {
            if album_needs_update(connection, id, album, year, path)? {
                connection.execute(
                    "UPDATE albums SET name = ?1, year = ?2, path = ?3 WHERE id_album = ?4",
                    params![album, year, path, id]
                )?;
                println!("Updated album: {}", album);
            }
            Ok(id)
        },
        None => {
            connection.execute(
                "INSERT INTO albums (name, year, path) VALUES (?1, ?2, ?3)",
                params![album, year, path]
            )?;
            let new_id = connection.last_insert_rowid();
            println!("Inserted new album: {}", album);
            Ok(new_id)
        }
    }
}

/// Checks if a song already exists in the "rolas" table, based on title, performer, and album.
/// Returns `true` if the song exists, `false` otherwise.
fn rola_exists(connection: &Connection, performer_id: i64, album_id: i64, title: &str) -> Result<bool> {
    let mut stmt = connection.prepare("SELECT EXISTS(SELECT 1 FROM rolas WHERE id_performer = ?1 AND id_album = ?2 AND title = ?3)")?;
    let exists: bool = stmt.query_row(params![performer_id, album_id, title], |row| row.get(0))?;
    Ok(exists)
}

/// Checks if a song already exists in the "rolas" table and if its metadata matches the new data.
/// If there are changes, it returns `true` to indicate that an update is needed.
fn song_needs_update(
    connection: &Connection,
    performer_id: i64,
    album_id: i64,
    title: &str,
    track: i32,
    year: i32,
    genre: &str
) -> Result<bool> {
    let mut stmt = connection.prepare("SELECT track, year, genre FROM rolas WHERE id_performer = ?1 AND id_album = ?2 AND title = ?3")?;
    let result: Option<(i32, i32, String)> = stmt.query_row(params![performer_id, album_id, title], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    }).optional()?;

    if let Some((db_track, db_year, db_genre)) = result {
        Ok(track != db_track || year != db_year || genre != db_genre)
    } else {
        Ok(false)
    }
}

/// Inserts a new track (song) into the "rolas" table only if it doesn't already exist.
/// This function adds a new track with the associated performer and album IDs, title, track number, year, and genre.
fn insert_or_update_rola(
    connection: &Connection,
    performer_id: i64,
    album_id: i64,
    title: &str,
    track: i32,
    year: i32,
    genre: &str,
    path: &str
) -> Result<()> {
    if !rola_exists(connection, performer_id, album_id, title)? {
        connection.execute(
            "INSERT INTO rolas (id_performer, id_album, title, track, year, genre, path) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![performer_id, album_id, title, track, year, genre, path]
        )?;
        println!("Inserted new song: {}", title);
    } else if song_needs_update(connection, performer_id, album_id, title, track, year, genre)? {
        connection.execute(
            "UPDATE rolas SET track = ?1, year = ?2, genre = ?3, path = ?4
             WHERE id_performer = ?5 AND id_album = ?6 AND title = ?7",
            params![track, year, genre, path, performer_id, album_id, title]
        )?;
        println!("Updated existing song: {}", title);
    }
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
    let path = tag_map.get("Path").unwrap();
    let album_path = tag_map.get("AlbumPath").unwrap(); 

    let performer_id = insert_or_update_performer(connection, artist)?;

    let album_id = insert_or_update_album(connection, album, year, album_path)?;

    insert_or_update_rola(connection, performer_id, album_id, title, track_number, year, genre, path)?;

    Ok(())
}
