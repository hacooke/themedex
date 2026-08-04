use rusqlite::Connection;

use crate::{
    common::test_fixtures::catppuccin_mocha_json,
    database::{models::ColorScheme, utils::HasConnection},
};

pub struct TestDb {
    pub conn: Option<Connection>,
    path: String,
}

impl HasConnection for TestDb {
    fn conn(&self) -> &Connection {
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
}

impl Drop for TestDb {
    fn drop(&mut self) {
        if let Some(connection) = self.conn.take() {
            connection.close().unwrap();
        }
        std::fs::remove_file(&self.path).unwrap();
    }
}

pub fn create_test_color_scheme(name: &str) -> ColorScheme {
    ColorScheme::new(name.to_string(), catppuccin_mocha_json())
}
