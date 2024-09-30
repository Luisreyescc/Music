extern crate dirs;
use crate::model::database_config::config_file; 
use std::fs;
use std::io;
use std::path::PathBuf;
use rusqlite::{Connection, Result};

fn create_database_file() -> io::Result<PathBuf> {
    let config_dir = config_file::create_config_dir()?;
    let file_path = config_dir.join("database.db");

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(&file_path)?;

    Ok(file_path)
}

pub fn create_database_connection() -> Result<Connection> {
    let file_path = match create_database_file() {
        Ok(path) => path,
        Err(e) => return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e))),
    };

    let connection = Connection::open(file_path)?;
    Ok(connection)
}

fn create_table_types(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS types (
            id_type       INTEGER PRIMARY KEY,
            description   TEXT
        )", ())?;

    Ok(())
}

fn create_table_performers(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS performers (
            id_performer       INTEGER PRIMARY KEY,
            id_type            INTEGER,
            name               TEXT,
            FOREIGN KEY   (id_type) REFERENCES types(id_type)
        )", ())?;

    Ok(())
}

fn create_table_persons(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS persons (
            id_person       INTEGER PRIMARY KEY,
            stage_name      TEXT,
            real_name       TEXT,
            birth_date      TEXT,
            death_date      TEXT 
        )", ())?;

    Ok(())
}

fn create_table_groups(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id_person       INTEGER PRIMARY KEY,
            name            TEXT,
            start_date      TEXT,
            end_date        TEXT 
        )", ())?;

    Ok(())
}

fn create_table_in_group(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS in_group (
            id_person       INTEGER,
            id_group        INTEGER,
            PRIMARY KEY (id_person, id_group),
            FOREIGN KEY (id_person) REFERENCES persons(id_person),
            FOREIGN KEY (id_group) REFERENCES groups(id_group)
        )", ())?;

    Ok(())
}

fn create_table_albums(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS albums (
            id_album        INTEGER PRIMARY KEY,
            path            TEXT,
            name            TEXT,
            year            INTEGER
        )", ())?;

    Ok(())
}

fn create_table_rolas(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS rolas (
            id_rola         INTEGER PRIMARY KEY,
            id_performer    INTEGER,
            id_album        INTEGER,
            path            TEXT,
            title           TEXT,
            track           INTEGER,
            year            INTEGER,
            genre           TEXT,
            FOREIGN KEY (id_performer) REFERENCES performers(id_performer),
            FOREIGN KEY (id_album) REFERENCES albums(id_album)
        )", ())?;

    Ok(())
}

pub fn create_all_tables(connection: &Connection) -> Result<()> {
    create_table_types(connection)?;
    create_table_performers(connection)?;
    create_table_persons(connection)?;
    create_table_groups(connection)?;
    create_table_in_group(connection)?;
    create_table_albums(connection)?;
    create_table_rolas(connection)?;

    Ok(())
}
