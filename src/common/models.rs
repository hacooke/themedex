use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type Color = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SchemeSystem {
    #[default]
    Base16,
    Base24,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SchemeVariant {
    Light,
    #[default]
    Dark,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SchemeJson {
    pub name: String,
    pub system: SchemeSystem,
    pub variant: SchemeVariant,
    pub palette: ColorPalette,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Color>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tools: HashMap<String, Color>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ColorPalette {
    pub color00: Color,
    pub color01: Color,
    pub color02: Color,
    pub color03: Color,
    pub color04: Color,
    pub color05: Color,
    pub color06: Color,
    pub color07: Color,
    pub color08: Color,
    pub color09: Color,
    pub color10: Color,
    pub color11: Color,
    pub color12: Color,
    pub color13: Color,
    pub color14: Color,
    pub color15: Color,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color16: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color17: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color18: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color19: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color20: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color21: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color22: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color23: Option<Color>,
}

/// Defines aliases for special colors
///
/// Can be given in the "extra" field of the scheme JSON.
/// Defaults if not provided taken from the Base16 framework:
/// https://github.com/chriskempson/base16/blob/main/styling.md
impl SchemeJson {
    pub fn background(&self) -> &Color {
        self.extra
            .get("background")
            .unwrap_or(&self.palette.color00)
    }

    pub fn lighter_background(&self) -> &Color {
        self.extra
            .get("lighter_background")
            .unwrap_or(&self.palette.color01)
    }

    pub fn selection_background(&self) -> &Color {
        self.extra
            .get("selection_background")
            .unwrap_or(&self.palette.color02)
    }

    pub fn dark_foreground(&self) -> &Color {
        self.extra
            .get("dark_foreground")
            .unwrap_or(&self.palette.color04)
    }

    pub fn foreground(&self) -> &Color {
        self.extra
            .get("foreground")
            .unwrap_or(&self.palette.color05)
    }

    pub fn light_foreground(&self) -> &Color {
        self.extra
            .get("light_foreground")
            .unwrap_or(&self.palette.color06)
    }

    pub fn light_background(&self) -> &Color {
        self.extra
            .get("light_background")
            .unwrap_or(&self.palette.color07)
    }
}
