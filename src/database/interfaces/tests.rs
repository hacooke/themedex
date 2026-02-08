use super::*;
use crate::database::test_structures::TestDb;
use std::fs;

fn create_test_color_scheme() -> ColorScheme {
    let file_contents = fs::read_to_string("tests/schemes/catppuccin-mocha.json").unwrap();
    let colors = serde_json::from_str(&file_contents).unwrap();
    ColorScheme::new(String::from("test"), colors)
}

#[test]
fn test_create_and_select_color_scheme() -> Result<(), ThemedexError> {
    // Arrange
    let db = TestDb::new("test_create_and_select_color_scheme.db");
    // Act
    let mut color_scheme = create_test_color_scheme();
    color_scheme.insert_to_db(db.conn())?;
    assert!(color_scheme.id.unwrap() > 0);
    let selected = ColorScheme::select_by_id(color_scheme.id.unwrap(), db.conn())?;
    // Assert
    assert_eq!(selected.id, color_scheme.id);
    assert_eq!(selected.name, color_scheme.name);
    assert_eq!(&selected.colors, &color_scheme.colors);
    Ok(())
}

#[test]
fn test_create_and_select_wallpaper_null_default() -> Result<(), ThemedexError> {
    // Arrange
    let db = TestDb::new("test_create_and_select_wallpaper_null_default");
    // Act
    let mut wallpaper = Wallpaper::new(String::from("test"), String::from("testpath"));
    wallpaper.insert_to_db(db.conn())?;
    assert!(wallpaper.id.unwrap() > 0);
    let selected = Wallpaper::select_by_id(wallpaper.id.unwrap(), db.conn())?;
    // Assert
    assert_eq!(selected.id, wallpaper.id);
    assert_eq!(selected.name, wallpaper.name);
    assert_eq!(selected.path, wallpaper.path);
    assert_eq!(selected.default_version_id, wallpaper.default_version_id);
    Ok(())
}

#[test]
fn test_create_and_select_wallpaper_version() -> Result<(), ThemedexError> {
    // Arrange
    let db = TestDb::new("test_create_and_select_wallpaper_version");
    let mut color_scheme = create_test_color_scheme();
    color_scheme.insert_to_db(db.conn())?;
    let mut wallpaper = Wallpaper::new(String::from("test"), String::from("testpath"));
    wallpaper.insert_to_db(db.conn())?;
    assert!(color_scheme.id.unwrap() > 0);
    assert!(wallpaper.id.unwrap() > 0);
    // Act
    let mut wallpaper_version = WallpaperVersion::new(&wallpaper, &color_scheme)?;
    wallpaper_version.insert_to_db(db.conn())?;
    let selected = WallpaperVersion::select_by_id(wallpaper.id.unwrap(), db.conn())?;
    // Assert
    assert_eq!(selected.id, wallpaper_version.id);
    assert_eq!(selected.wallpaper_id, wallpaper_version.wallpaper_id);
    assert_eq!(selected.color_scheme_id, wallpaper_version.color_scheme_id);
    Ok(())
}
