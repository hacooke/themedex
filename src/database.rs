pub mod interfaces;
pub mod models;
pub mod query;
pub mod utils;

use std::path::Path;

use rusqlite::{Connection, Result};

pub fn init_db(path: impl AsRef<Path>) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(include_str!("../db/schema.sql"))?;
    Ok(conn)
}

#[cfg(test)]
pub mod test_structures;
