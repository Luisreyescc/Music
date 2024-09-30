extern crate dirs;
use rusqlite::{Connection, Result};

/// Creates the `types` table if it doesn't exist.
/// Stores performer types with an ID and description.
fn create_table_types(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS types (
            id_type       INTEGER PRIMARY KEY,
            description   TEXT
        )", ())?;

    Ok(())
}

/// Creates the `performers` table if it doesn't exist.
/// Stores performers, linked to `types` via `id_type`.
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

/// Creates the `persons` table if it doesn't exist.
/// Stores individual person details (stage name, real name, etc.).
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

/// Creates the `groups` table if it doesn't exist.
/// Stores group details such as name and active years.
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

/// Creates the `in_group` table if it doesn't exist.
/// Links persons and groups.
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

/// Creates the `albums` table if it doesn't exist.
/// Stores album details like path, name, and year.
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

/// Creates the `rolas` table if it doesn't exist.
/// Stores song details, linking to `performers` and `albums`.
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

/// Creates all necessary tables in the database.
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
