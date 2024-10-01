/// # `database_config` Module
///
/// The `database` module contains functionality for managing configuration and files
/// related to persistent data storage. It is responsible for loading and saving user settings,
/// among other tasks related to persistence.
///
/// ## Submodules
///
/// - `config`: Implements functions for reading and writing configuration files.
/// - `database_tables`: Implement the creation of the database tables.

pub mod config; 
pub mod database_tables;
pub mod populate_db;
