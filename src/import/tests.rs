use std::io::ErrorKind;

use crate::database::test_structures::TestDb;

use super::*;

#[test]
fn test_import_color_scheme_json() -> Result<(), ThemedexError> {
    let agent = ImportAgent {
        wallpaper_directory: "./tests/wallpapers".into(),
        conn: TestDb::new("test_import_color_scheme_json.db"),
    };
    agent.import_color_scheme_json("./tests/schemes/catppuccin-mocha.json", None, false)?;
    Ok(())
}

#[test]
fn test_import_color_scheme_no_file() {
    let agent = ImportAgent {
        wallpaper_directory: "./tests/wallpapers".into(),
        conn: TestDb::new("test_import_color_scheme_no_file.db"),
    };
    match agent.import_color_scheme_json("./tests/schemes/noscheme.json", None, false) {
        Err(ThemedexError::Io(e)) if e.kind() == ErrorKind::NotFound => (),
        _ => panic!("Not expected error"),
    };
}

#[test]
fn test_import_existing_color_scheme() {
    let agent = ImportAgent {
        wallpaper_directory: "./tests/wallpapers".into(),
        conn: TestDb::new("test_import_existing_color_scheme.db"),
    };
    agent
        .import_color_scheme_json("./tests/schemes/catppuccin-mocha.json", None, false)
        .unwrap();
    match agent.import_color_scheme_json("./tests/schemes/catppuccin-mocha.json", None, false) {
        Err(ThemedexError::Internal(e)) if e.to_string().contains("already existsx") => (),
        _ => panic!("Not expected error"),
    };
}
