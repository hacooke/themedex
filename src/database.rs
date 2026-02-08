pub mod interfaces;
pub mod models;
pub mod utils;

use rusqlite::{Connection, Result};

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(include_str!("../db/schema.sql"))?;
    Ok(conn)
}

#[cfg(test)]
pub mod test_structures;
