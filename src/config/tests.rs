use super::*;

#[test]
fn test_partial_config_construction() {
    let config: ThemedexConfig =
        serde_json::from_str("{\"directories\": {\"wallpaper_directory\": \"user_walls\"}}")
            .expect("");
    let defaults = ThemedexConfig::default();
    assert_eq!(config.directories.wallpaper_directory, "user_walls");
    assert_eq!(
        config.directories.database_directory,
        defaults.directories.database_directory
    );
}
