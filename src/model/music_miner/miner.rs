use crate::model::music_miner::song_settings; 
extern crate id3;
use std::collections::HashMap;
use id3::{Tag, Version};
use std::fs;
use std::path::Path;

/// # Extract function
///
/// Recursively traverses the specified directory and processes each file found in it and its subdirectories.
/// For each MP3 file, it calls the `process_song()` function to extract and handle the file's metadata.
/// The function collects the metadata for all valid MP3 files in the directory and returns it as a vector of hash maps.
/// Each hash map contains information like the file path and album path. If an error occurs while reading a file or directory, a message is printed, and the process continues.
///
/// # Arguments
/// * `mp3_dir_path` - A string slice that holds the path to the directory containing MP3 files.
///
/// # Returns
/// * `Vec<HashMap<String, String>>` - A vector of hash maps where each map contains metadata about an MP3 file.
///
/// # Panics
/// Panics if the directory cannot be read.
pub fn extract(mp3_dir_path: &str) -> Vec<HashMap<String, String>> {
    let mut extracted_data = Vec::new();
    
    visit_dirs(Path::new(mp3_dir_path), &mut extracted_data);

    extracted_data
}

/// # Visit Directories function
///
/// Recursively traverses the directory structure starting from the provided `dir` path.
/// For each subdirectory, the function calls itself to continue traversing.
/// For each file, the function checks if it's an MP3 file by calling the `process_song()` function.
/// If the file is an MP3, it extracts metadata and adds it to the `extracted_data` vector.
/// The metadata includes the file path and the album path (the parent directory).
/// If a file is not a valid MP3 file, a warning is printed.
///
/// # Arguments
/// * `dir` - A reference to a `Path` that represents the directory to be traversed.
/// * `extracted_data` - A mutable reference to a vector of hash maps, which stores metadata for each MP3 file found.
///
/// # Panics
/// Panics if the directory cannot be read.
fn visit_dirs(dir: &Path, extracted_data: &mut Vec<HashMap<String, String>>) {
    if dir.is_dir() {
        let music = fs::read_dir(dir).expect("Could not read the directory.");
        
        for song in music {
            let song = song.expect("Could not read the entry.");
            let path = song.path();
            
            if path.is_dir() {
                visit_dirs(&path, extracted_data);
            } else if path.is_file() {
                if let Some(mut tag_map) = process_song(&path) {
                    let album_path = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
                    tag_map.insert("Path".to_string(), path.to_str().unwrap().to_string());
                    tag_map.insert("AlbumPath".to_string(), album_path.to_str().unwrap().to_string());
                    extracted_data.push(tag_map);
                } else {
                    println!("{:?} is not a valid MP3 file", path);
                }
            }
        }
    }
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
