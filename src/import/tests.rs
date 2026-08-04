use std::io::ErrorKind;

use crate::{common::test_fixtures::TestDir, database::test_structures::{TestDb, create_test_color_scheme}};

use super::*;

use std::fs::metadata;

#[test]
fn test_import_color_scheme_json() -> Result<(), ThemedexError> {
    // Arrange
    let agent = ImportAgent {
        wallpaper_directory: "./tests/wallpapers".into(),
        conn: TestDb::new("test_import_color_scheme_json.db"),
    };
    // Act
    let color_scheme = agent.import_color_scheme_json("./tests/schemes/catppuccin-mocha.json", None, false)?;
    // Assert
    assert_eq!(color_scheme.name, "catppuccin-mocha");
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
        Err(ThemedexError::Internal(e)) if e.to_string().contains("already exists") => (),
        Err(e) => panic!("Wrong error? {e}"),
        Ok(_) => panic!("Expected error but got Ok"),
    };
}

#[test]
fn test_import_wallpaper_and_version() {
    // Arrange
    let wallsdir = TestDir::new("wallpapers_test_import_wallpaper_and_version");
    let agent = ImportAgent {
        wallpaper_directory: dbg!(wallsdir.to_string()),
        conn: TestDb::new("test_import_wallpaper_and_version.db"),
    };
    let mut color_scheme = create_test_color_scheme("test-color-scheme");
    color_scheme.insert_to_db(agent.conn.conn()).unwrap();
    // Act
    let (wallpaper, version) = agent
        .import_wallpaper_and_version(
            "./tests/importwalls/test-wall.png",
            &color_scheme.name,
            None,
            false,
        )
        .unwrap();
    // Assert
    assert_eq!(wallpaper.name, "test-wall");
    assert_eq!(absolute(&wallpaper.path).unwrap(), absolute(wallsdir.path.join("test-wall")).unwrap());
    assert!(metadata(&wallpaper.path).unwrap().is_dir());
    assert_eq!(version.wallpaper_id, wallpaper.id.unwrap());
    assert_eq!(version.color_scheme_id, color_scheme.id.unwrap());
    assert_eq!(wallpaper.default_version_id, version.id);
}

#[test]
fn test_import_wallpaper_and_version_with_name() {
    // Arrange
    let wallsdir = TestDir::new("wallpapers_test_import_wallpaper_and_version_with_name");
    let agent = ImportAgent {
        wallpaper_directory: dbg!(wallsdir.to_string()),
        conn: TestDb::new("test_import_wallpaper_and_version_with_name.db"),
    };
    let mut color_scheme = create_test_color_scheme("test-color-scheme");
    color_scheme.insert_to_db(agent.conn.conn()).unwrap();
    // Act
    let (wallpaper, version) = agent
        .import_wallpaper_and_version(
            "./tests/importwalls/test-wall.png",
            &color_scheme.name,
            Some("custom-name"),
            false,
        )
        .unwrap();
    // Assert
    assert_eq!(wallpaper.name, "custom-name");
    assert_eq!(absolute(&wallpaper.path).unwrap(), absolute(wallsdir.path.join("custom-name")).unwrap());
    assert!(metadata(&wallpaper.path).unwrap().is_dir());
    assert_eq!(version.wallpaper_id, wallpaper.id.unwrap());
    assert_eq!(version.color_scheme_id, color_scheme.id.unwrap());
}

#[test]
fn test_import_wallpaper_and_version_overwrite() {
    // Arrange
    let wallsdir = TestDir::new("wallpapers_test_import_wallpaper_and_version_overwrite");
    let agent = ImportAgent {
        wallpaper_directory: dbg!(wallsdir.to_string()),
        conn: TestDb::new("test_import_wallpaper_and_version_overwrite.db"),
    };
    let mut color_scheme_1 = create_test_color_scheme("test-color-scheme");
    color_scheme_1.insert_to_db(agent.conn.conn()).unwrap();
    let mut color_scheme_2 = create_test_color_scheme("test-other-color-scheme");
    color_scheme_2.insert_to_db(agent.conn.conn()).unwrap();
    agent
        .import_wallpaper_and_version(
            "./tests/importwalls/test-wall.png",
            &color_scheme_1.name,
            Some("custom-name"),
            false,
        )
        .unwrap();
    // Act
    let (wallpaper, version) = agent
        .import_wallpaper_and_version(
            "./tests/importwalls/test-wall.png",
            &color_scheme_2.name,
            Some("custom-name"),
            true,
        )
        .unwrap();
    // Assert
    assert_eq!(wallpaper.name, "custom-name");
    assert_eq!(absolute(&wallpaper.path).unwrap(), absolute(wallsdir.path.join("custom-name")).unwrap());
    assert!(metadata(&wallpaper.path).unwrap().is_dir());
    assert_eq!(version.wallpaper_id, wallpaper.id.unwrap());
    assert_eq!(version.color_scheme_id, color_scheme_2.id.unwrap());
}
