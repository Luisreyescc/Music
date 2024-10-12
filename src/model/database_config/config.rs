extern crate dirs;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Creates the `/home/user/.config/musicmanager/` directory if it doesn't exist.
///
/// This function locates the user's configuration directory (typically `~/.config/` on Linux,
/// `~/Library/Application Support/` on macOS, and `C:\Users\Username\AppData\Roaming\` on Windows)
/// and appends a subdirectory named `musicmanager` to it. If the directory does not exist,
/// it will be created using `fs::create_dir_all`.
///
/// # Returns
///
/// - `Ok(PathBuf)` containing the path to the `musicmanager` directory if successful.
/// - `Err(io::Error)` if the configuration directory could not be found or if there was an error 
///   while creating the directory.
pub fn create_config_dir() -> io::Result<PathBuf> {
    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push("musicmanager");
        fs::create_dir_all(&config_dir)?; 
        Ok(config_dir)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Could not find configuration directory."))
    }
}

/// Creates the `database.db` file inside the `~/.local/share/musicmanager/` directory if it doesn't exist.
///
/// This function ensures that a `database.db` file is created in the `musicmanager` directory 
/// within the user's configuration path. If the file already exists, it will not be modified.
/// Otherwise, a new file will be created using `fs::OpenOptions`.
///
/// This is particularly useful for initializing a database for storing application data.
///
/// # Returns
///
/// - `Ok(PathBuf)` containing the path to the `database.db` file if it was created successfully
///   or if it already exists.
/// - `Err(io::Error)` if there was an error creating the directory or the file.
pub fn create_database_file() -> io::Result<PathBuf> {
    let config_dir = get_local_dir()?;
    let file_path = config_dir.join("database.db");

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&file_path)?;

    Ok(file_path)
}

/// Retrieves or creates the `~/.local/share/musicmanager/` directory for the user's application data.
///
/// This function ensures that a `musicmanager` directory is created inside the user's local
/// data directory (e.g., `~/.local/share/` on Linux). If the directory does not exist, it will
/// be created. This directory is typically used to store application data that persists
/// between sessions (e.g., databases, caches, etc.).
///
/// # Returns
///
/// - `Ok(PathBuf)` containing the path to the `musicmanager` directory if successful.
/// - `Err(io::Error)` if the local data directory could not be found or created.
pub fn get_local_dir() -> io::Result<PathBuf> {
    if let Some(mut local_share_dir) = dirs::data_dir() {
        local_share_dir.push("musicmanager");
        fs::create_dir_all(&local_share_dir)?; 
        Ok(local_share_dir)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Could not find configuration directory."))
    }
}
