use rusqlite::Connection;

pub trait HasConnection {
    fn connection(&self) -> &Connection;
}
