use rusqlite::Connection;

use crate::database::utils::HasConnection;

pub struct TestDb {
    pub conn: Option<Connection>,
    path: String,
}

impl HasConnection for TestDb {
    fn connection(&self) -> &Connection {
        self.conn.as_ref().unwrap()
    }
}

impl TestDb {
    pub fn new(path: &str) -> Self {
        Self {
            conn: Some(crate::database::init_db(path).unwrap()),
            path: path.to_string(),
        }
    }

    pub fn conn(&self) -> &Connection {
        self.conn.as_ref().unwrap()
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        if let Some(connection) = self.conn.take() {
            connection.close().unwrap();
        }
        std::fs::remove_file(&self.path).unwrap();
    }
}
