use crate::config::test_fixtures::make_config;

use super::*;

#[test]
fn test_partial_config_construction() {
    let config: ThemedexConfig =
        serde_json::from_str("{\"directories\": {\"wallpaper_directory\": \"user_walls\"}}")
            .expect("");
    let defaults = ThemedexConfig::default();
    assert_eq!(
        config.directories.wallpaper_directory.display().to_string(),
        "user_walls"
    );
    assert_eq!(
        config.directories.database_path,
        defaults.directories.database_path
    );
}

#[test]
fn test_validate_config_valid() {
    let config = make_config();
    config.directories.validate_and_create_paths().expect("");
}

#[test]
fn test_validate_config_invalid_directory() {
    let config = DirectoryConfig {
        wallpaper_directory: "/\0".into(),
        database_path: "tests/themedex.db".into(),
        template_directory: "tests/templates".into(),
        rendered_config_directory: "tests/rendered".into(),
    };
    config.validate_and_create_paths().expect_err("");
}

#[test]
fn test_validate_config_invalid_database() {
    let config = DirectoryConfig {
        wallpaper_directory: "tests/wallpapers".into(),
        database_path: "/\0/test.db".into(),
        template_directory: "tests/templates".into(),
        rendered_config_directory: "tests/rendered".into(),
    };
    config.validate_and_create_paths().expect_err("");
}
