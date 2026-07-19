use rusqlite::Connection;

pub trait HasConnection {
    fn conn(&self) -> &Connection;
}

pub struct ThemedexDb {
    conn: Option<Connection>,
}

impl ThemedexDb {
    pub fn new(path: &str) -> Self {
        Self {
            conn: Some(crate::database::init_db(path).unwrap()),
        }
    }
}

impl HasConnection for ThemedexDb {
    fn conn(&self) -> &Connection {
        self.conn.as_ref().unwrap()
    }
}
