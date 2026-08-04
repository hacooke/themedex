use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::common::ThemedexError;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ThemedexConfig {
    pub directories: DirectoryConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DirectoryConfig {
    pub wallpaper_directory: String,
    pub database_directory: String,
    pub template_directory: String,
    pub rendered_config_directory: PathBuf,
}

impl Default for DirectoryConfig {
    fn default() -> Self {
        Self {
            wallpaper_directory: "walls".into(),
            database_directory: "db".into(),
            template_directory: "templates".into(),
            rendered_config_directory: "rendered".into(),
        }
    }
}


impl ThemedexConfig {
    pub fn from_file(config_file: impl AsRef<Path>) -> Result<Self, ThemedexError> {
        let file_contents = fs::read_to_string(config_file)?;
        Ok(toml::from_str(&file_contents)?)
    }
// TODO validate config has valid paths?
// TODO make config directories after reading config?
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod test_fixtures;
