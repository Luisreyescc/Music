extern crate id3;
use std::collections::HashMap;
use id3::Tag;

pub fn assign_tag(tag: &Tag) -> HashMap<String, String>{
    let mut map: HashMap<String, String> = HashMap::new();
    
    if let Some(artist) = tag.artist() {
       map.insert("Artist".to_string(), artist.to_string()); 
    }

    if let Some(title) = tag.title() {
       map.insert("Title".to_string(), title.to_string()); 
    }

    if let Some(album) = tag.album() {
       map.insert("Album".to_string(), album.to_string()); 
    }

    if let Some(year) = tag.year() {
       map.insert("Year".to_string(), year.to_string()); 
    }

    if let Some(genre) = tag.genre() {
       map.insert("Genre".to_string(), genre.to_string()); 
    }

    if let Some(track_number) = tag.track() {
       map.insert("Track Number".to_string(), track_number.to_string()); 
    }

    map
}
