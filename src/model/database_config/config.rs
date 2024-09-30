extern crate dirs;
use std::fs;
use std::io;
use std::path::PathBuf;
use rusqlite::{Connection, Result};

/// Creates the `/home/user/.config/musicmanager/` directory if it doesn't exist.
///
/// This function locates the user's configuration directory (typically `~/.config/`)
/// and appends a subdirectory named `musicmanager` to it. If the directory does not
/// exist, it will be created. 
///
/// # Returns
///
/// - `Ok(PathBuf)` containing the path to the `musicmanager` directory.
/// - `Err(io::Error)` if the configuration directory could not be found or there was an error creating the directory.
pub fn create_config_dir() -> io::Result<PathBuf> {
    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push("musicmanager");
        fs::create_dir_all(&config_dir)?; 
        Ok(config_dir)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Could not find configuration directory."))
    }
}

/// Creates the `Config.TOML` file inside the `~/.config/musicmanager/` directory if it doesn't exist.
///
/// This function ensures that the `Config.TOML` file is created within the `musicmanager`
/// directory inside the user's configuration path. If the file already exists, it is not
/// modified. If it doesn't exist, an empty file is created.
///
/// # Returns
///
/// - `Ok(())` if the file is successfully created or already exists.
/// - `Err(io::Error)` if there is an error finding the directory or creating the file.
pub fn create_config_file() -> io::Result<()> {
    match create_config_dir() {
        Ok(config_dir) => {
            let file_path = config_dir.join("Config.TOML");

            let _file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(&file_path)?;

            Ok(())
        }
        Err(e) => {
            Err(e) 
        }
    }

}

/// Creates the `database.db` file inside the `~/.config/musicmanager/` directory if it doesn't exist.
///
/// This function ensures that the `database.db` file is created within the `musicmanager`
/// directory inside the user's configuration path. If the file already exists, it is not
/// modified. If it doesn't exist, an empty file is created for use by SQLite.
///
/// # Returns
///
/// - `Ok(PathBuf)` containing the path to the `database.db` file if it was created successfully or already exists.
/// - `Err(io::Error)` if there was an error creating the file or directory.
fn create_database_file() -> io::Result<PathBuf> {
    let config_dir = create_config_dir()?;
    let file_path = config_dir.join("database.db");

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(&file_path)?;

    Ok(file_path)
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
