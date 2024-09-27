use crate::model::song_settings; 
extern crate id3;
use std::collections::HashMap;
use id3::{Tag, Version};
use std::fs;

pub fn extract(mp3_dir_path: &str) {
    let songs = fs::read_dir(mp3_dir_path).expect("Could not read the directory.");

    for song in songs {
        let song = song.expect("Could not read the song.");
        let path = song.path();

        if process_song(&path).is_err() {
            println!("{:?} is not a valid MP3 file", path);
        }
    }
}

fn process_song(path: &std::path::Path) -> Result<(), ()> {
    if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("mp3") {
        return Err(());
    }

    println!("Processing file {:?}", path);

    match Tag::read_from_path(path) {
        Ok(tag) => {
            if tag.version() == Version::Id3v24 {
                println!("ID3v2.4 tag found in: {:?} \n", path);
                song_settings::assign_tag(&tag);
                print_tag_info(song_settings::assign_tag(&tag));
            } else {
                println!("Tag is not ID3v2.4, file: {:?} \n", path);
            }
        }
        Err(e) => {
            println!("Failed to read the tags in {:?}: {:?} \n", path, e);
        }
    }

    Ok(())
}

fn print_tag_info(map: HashMap<String, String>) {
    for (tag, tag_content) in map.iter() {
        println!("{tag}: {tag_content} \n"); 
    }
}
