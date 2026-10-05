use serde::{Deserialize, Serialize};

use crate::common::{ThemedexError, models::SchemeJson};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorScheme {
    pub id: Option<u32>,
    pub name: String,
    pub colors: SchemeJson,
}

#[derive(Debug)]
pub struct Wallpaper {
    pub id: Option<u32>,
    pub name: String,
    pub path: String,
    pub default_version_id: Option<u32>,
}

#[derive(Debug)]
pub struct WallpaperVersion {
    pub id: Option<u32>,
    pub wallpaper_id: u32,
    pub color_scheme_id: u32,
    pub filename: String,
}

#[derive(Debug)]
pub struct WallpaperVersionNames {
    pub wallpaper_name: String,
    pub color_scheme_name: String,
}

impl ColorScheme {
    pub fn new(name: String, colors: SchemeJson) -> Self {
        Self {
            id: None,
            name,
            colors,
        }
    }
}

impl Wallpaper {
    pub fn new(name: String, path: String) -> Self {
        Self {
            id: None,
            name,
            path,
            default_version_id: None,
        }
    }
}

impl WallpaperVersion {
    pub fn new(
        wallpaper: &Wallpaper,
        color_scheme: &ColorScheme,
        filename: String,
    ) -> Result<Self, ThemedexError> {
        let Some(wallpaper_id) = wallpaper.id else {
            return Err(ThemedexError::Internal(String::from(
                "Tried to create wallpaper version from wallpaper which is not in database.",
            )));
        };
        let Some(color_scheme_id) = color_scheme.id else {
            return Err(ThemedexError::Internal(String::from(
                "Tried to create wallpaper version from color scheme which is not in database.",
            )));
        };
        Ok(Self {
            id: None,
            wallpaper_id,
            color_scheme_id,
            filename,
        })
    }
}
