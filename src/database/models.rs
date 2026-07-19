use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::ThemedexError;

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
}

#[derive(Debug)]
pub struct WallpaperVersionNames {
    pub wallpaper_name: String,
    pub color_scheme_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SchemeJson {
    pub name: String,
    pub system: SchemeSystem,
    pub variant: SchemeVariant,
    pub palette: ColorPalette,
    pub extra: HashMap<String, String>,
    pub tools: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ColorPalette {
    color00: String,
    color01: String,
    color02: String,
    color03: String,
    color04: String,
    color05: String,
    color06: String,
    color07: String,
    color08: String,
    color09: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
    color16: Option<String>,
    color17: Option<String>,
    color18: Option<String>,
    color19: Option<String>,
    color20: Option<String>,
    color21: Option<String>,
    color22: Option<String>,
    color23: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SchemeSystem {
    Base16,
    Base24,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SchemeVariant {
    Light,
    Dark,
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
    pub fn new(wallpaper: &Wallpaper, color_scheme: &ColorScheme) -> Result<Self, ThemedexError> {
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
        })
    }
}
