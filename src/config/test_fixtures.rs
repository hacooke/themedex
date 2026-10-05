use super::*;

pub fn make_config() -> ThemedexConfig {
    ThemedexConfig {
        directories: DirectoryConfig {
            wallpaper_directory: "tests/wallpapers".into(),
            database_path: "tests/themedex.db".into(),
            template_directory: "tests/templates".into(),
            rendered_config_directory: "tests/rendered".into(),
        },
        set_wallpaper_command: Default::default(),
    }
}
