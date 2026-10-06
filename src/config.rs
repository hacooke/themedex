use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};

use crate::common::ThemedexError;

// TODO add config option for wallpaper symlink location.
// If provided, whenever wallpaper is set, update the symlink.
// Also, reorganise command config, e.g.
//
// [commands]
//
// set_wallpaper_command = ["swww", "img", "{{path}}"]
// post_scheme_change_commands = [["killall" "-SIGUSR2", "waybar"]]
// wallpaper_symlink_location = "/home/harry/documents/photos/wallpapers/current.png

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ThemedexConfig {
    pub set_wallpaper_command: WallpaperCommandConfig,
    pub scheme_change_commands: Vec<Vec<String>>,
    pub directories: DirectoryConfig,
}

impl ThemedexConfig {
    pub fn from_file(config_file: impl AsRef<Path>) -> Result<Self, ThemedexError> {
        let file_contents = fs::read_to_string(config_file)?;
        let config: Self = toml::from_str(&file_contents)?;
        config.directories.validate_and_create_paths()?;
        Ok(config)
    }
}

impl Display for ThemedexConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config_toml = toml::to_string(self).map_err(|_| std::fmt::Error {})?;
        write!(f, "{}", config_toml)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct WallpaperCommandConfig {
    pub executable: String,
    pub arguments: Vec<String>,
}

impl Default for WallpaperCommandConfig {
    fn default() -> Self {
        Self {
            executable: "swww".into(),
            arguments: vec!["img".into()],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DirectoryConfig {
    /// Path to directory where themedex should store wallpaper image files
    pub wallpaper_directory: PathBuf,
    /// Path to themdex SQLite database file
    pub database_path: PathBuf,
    /// Path to directory where user templates should be read from
    pub template_directory: PathBuf,
    /// Path to directory where themedex should store redered template results
    pub rendered_config_directory: PathBuf,
}

impl Default for DirectoryConfig {
    fn default() -> Self {
        let config_dir = get_system_config_dir(Default::default()).unwrap_or(".".into());
        Self {
            wallpaper_directory: config_dir.join("wallpapers"),
            database_path: config_dir.join("themedex.db"),
            template_directory: config_dir.join("templates"),
            rendered_config_directory: config_dir.join("rendered"),
        }
    }
}

impl DirectoryConfig {
    pub(crate) fn validate_and_create_paths(&self) -> Result<(), ThemedexError> {
        // Create directories which themedex writes to
        fs::create_dir_all(&self.wallpaper_directory)?;
        fs::create_dir_all(&self.rendered_config_directory)?;
        match self.database_path.parent() {
            Some(parent) => fs::create_dir_all(parent)?,
            None => {
                return Err(ThemedexError::Internal(format!(
                    "Failed to identify parent path for provided database path: {}",
                    self.database_path.display()
                )));
            }
        }
        // Check for directories which themedex reads from
        if !self.template_directory.is_dir() {
            log::warn!("Template directory not found, themedex will not render any templates")
        }
        // Check DB file either does not yet exist or is valid DB
        if self.database_path.exists() {
            let conn =
                Connection::open_with_flags(&self.database_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
            conn.query_row("PRAGMA user_version;", [], |_| Ok(()))?;
        }
        Ok(())
    }
}

pub struct SystemConfigLocation {
    pub qualifier: &'static str,
    pub organisation: &'static str,
    pub application: &'static str,
    pub filename: &'static str,
}

impl Default for SystemConfigLocation {
    fn default() -> Self {
        Self {
            qualifier: "com",
            organisation: "themedex",
            application: "themedex",
            filename: "config.toml",
        }
    }
}

fn get_system_config_dir(options: SystemConfigLocation) -> Result<PathBuf, ThemedexError> {
    let os_project_dirs =
        ProjectDirs::from(options.qualifier, options.organisation, options.application);
    match os_project_dirs {
        Some(project_dirs) => Ok(project_dirs.config_dir().to_owned()),
        None => Err(ThemedexError::Internal(
            "Could not find system config path".to_owned(),
        )),
    }
}

pub fn get_config_file(
    cli_override: Option<String>,
    options: SystemConfigLocation,
) -> Result<PathBuf, ThemedexError> {
    if let Some(file_path) = cli_override {
        return Ok(file_path.into());
    }
    let filename = options.filename;
    Ok(get_system_config_dir(options)?.join(filename))
}

#[cfg(test)]
pub mod test_fixtures;
#[cfg(test)]
mod tests;
