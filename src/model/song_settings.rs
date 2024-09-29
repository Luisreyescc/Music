extern crate id3;
use std::collections::HashMap;
use id3::Tag;
use chrono::Datelike;

/// # Assign Tag function
///
/// Extracts key metadata from an ID3v2.4 tag of an MP3 file and stores it in a `HashMap`.
/// If a specific tag is missing, it assigns "Unknown" as the default value.
/// If the year is missing, it assigns the current year.
/// If the track number is missing, it assigns `0`.
///
/// # Arguments
/// * `tag` - A reference to the tag from which the metadata is extracted.
///
/// # Returns
/// A `HashMap<String, String>` containing key-value pairs for each tag field (artist, title, album, etc.).
pub fn assign_tag(tag: &Tag) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    
    let artist = tag.artist().unwrap_or("Unknown");
    map.insert("Artist".to_string(), artist.to_string());

    let title = tag.title().unwrap_or("Unknown");
    map.insert("Title".to_string(), title.to_string());

    let album = tag.album().unwrap_or("Unknown");
    map.insert("Album".to_string(), album.to_string());

    let current_year = chrono::Utc::now().year();
    let year = tag.year().unwrap_or(current_year);
    map.insert("Year".to_string(), year.to_string());

    let genre = tag.genre().unwrap_or("Unknown");
    map.insert("Genre".to_string(), genre.to_string());

    let track_number = tag.track().unwrap_or(0);
    map.insert("Track Number".to_string(), track_number.to_string());

    map
}
