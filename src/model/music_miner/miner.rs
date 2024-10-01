use crate::model::music_miner::song_settings; 
extern crate id3;
use std::collections::HashMap;
use id3::{Tag, Version};
use std::fs;

/// # Extract function
///
/// Traverses the given directory and processes each file in it.
/// For each MP3 file, it calls the `process_song()` function to handle the file's metadata.
/// If an error occurs while reading the directory or processing a file, it prints a message.
///
/// # Arguments
/// * `mp3_dir_path` - The path to the directory containing MP3 files.
///
/// # Panics
/// Panics if the directory cannot be read.
pub fn extract(mp3_dir_path: &str) -> Vec<HashMap<String, String>> {
    let songs = fs::read_dir(mp3_dir_path).expect("Could not read the directory.");
    let mut extracted_data = Vec::new();

    for song in songs {
        let song = song.expect("Could not read the song.");
        let path = song.path();

        if let Some(tag_map) = process_song(&path) {
            extracted_data.push(tag_map);
        } else {
            println!("{:?} is not a valid MP3 file", path);
        }
    }

    extracted_data
}

/// # Process Song function
///
/// Processes the metadata of a given MP3 file, checking if it has an ID3v2.4 tag.
/// If the tag is found, it extracts metadata such as artist, title, album, year, genre,
/// and track number by calling `assign_tag()`. 
///
/// # Arguments
/// * `path` - The path to the MP3 file to be processed.
///
/// # Returns
/// * `Ok(())` if the file was successfully processed.
/// * `Err(())` if the file is not a valid MP3 file or if it has no readable tags.
fn process_song(path: &std::path::Path) -> Option<HashMap<String, String>> {
    if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("mp3") {
        return None;
    }

    println!("Processing file {:?}", path);

    match Tag::read_from_path(path) {
        Ok(tag) => {
            if tag.version() == Version::Id3v24 {
                println!("ID3v2.4 tag found in: {:?} \n", path);
                let tag_map = song_settings::assign_tag(&tag);
                print_tag_info(&tag_map);
                return Some(tag_map);
            } else {
                println!("Tag is not ID3v2.4, file: {:?} \n", path);
            }
        }
        Err(e) => {
            println!("Failed to read the tags in {:?}: {:?} \n", path, e);
        }
    }

    None
}

fn print_tag_info(map: &HashMap<String, String>) {
    for (tag, tag_content) in map.iter() {
        println!("{tag}: {tag_content} \n"); 
    }
}
