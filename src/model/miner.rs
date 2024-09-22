extern crate id3;
use id3::{Tag, Version};
use std::fs;

pub fn extract(mp3_dir_path: &str) {
    let songs = fs::read_dir(mp3_dir_path).expect("Could not read the directory.");

    for song in songs {
        let song = song.expect("Could not read the song.");
        let path = song.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("mp3") {
            println!("Prosessing file {:?}", path);

            match Tag::read_from_path(&path) {
                Ok(tag) => {
                    if tag.version() == Version::Id3v24 {
                        println!("ID3v2.4 tag found in: {:?}", path);

                        if let Some(interpreter) = tag.artist() {
                            println!("Artist: {}", interpreter);
                        }

                        if let Some(title) = tag.title() {
                            println!("Title: {}", title);
                        }

                        if let Some(album) = tag.album() {
                            println!("Album: {}", album);
                        }

                    } else {
                        println!("Tag is not IP3v2.4, file: {:?}", path);
                    }
                } Err (e) => {
                    println!("Failed to read te tags in {:?}: {:?}", path, e);
                }
            }
        } else {
            println!("{:?} is not a MP3 valid file", path);
        }

    }

}
