use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use crate::common::models::{ColorPalette, SchemeJson, SchemeSystem, SchemeVariant};

pub struct TestDir {
    pub path: PathBuf,
}

impl Display for TestDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

impl TestDir {
    pub fn new(path: &str) -> Self {
        let full_path = Path::new("./tests").join(path);
        std::fs::create_dir_all(&full_path).unwrap();
        Self { path: full_path }
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

pub fn catppuccin_mocha_json() -> SchemeJson {
    let file_contents = fs::read_to_string("tests/schemes/catppuccin-mocha.json").unwrap();
    serde_json::from_str(&file_contents).unwrap()
}

pub fn tokyonight_storm_json() -> SchemeJson {
    let file_contents = fs::read_to_string("tests/schemes/tokyo-night-storm.json").unwrap();
    serde_json::from_str(&file_contents).unwrap()
}

pub fn make_palette() -> ColorPalette {
    ColorPalette {
        color00: "#000000".into(),
        color01: "#000001".into(),
        color02: "#000002".into(),
        color03: "#000003".into(),
        color04: "#000004".into(),
        color05: "#000005".into(),
        color06: "#000006".into(),
        color07: "#000007".into(),
        color08: "#000008".into(),
        color09: "#000009".into(),
        color10: "#000010".into(),
        color11: "#000011".into(),
        color12: "#000012".into(),
        color13: "#000013".into(),
        color14: "#000014".into(),
        color15: "#000015".into(),
        color16: Some("#000016".into()),
        color17: Some("#000017".into()),
        color18: Some("#000018".into()),
        color19: Some("#000019".into()),
        color20: Some("#000020".into()),
        color21: Some("#000021".into()),
        color22: Some("#000022".into()),
        color23: Some("#000023".into()),
    }
}

pub fn make_palette_minimal() -> ColorPalette {
    ColorPalette {
        color00: "#000000".into(),
        color01: "#000001".into(),
        color02: "#000002".into(),
        color03: "#000003".into(),
        color04: "#000004".into(),
        color05: "#000005".into(),
        color06: "#000006".into(),
        color07: "#000007".into(),
        color08: "#000008".into(),
        color09: "#000009".into(),
        color10: "#000010".into(),
        color11: "#000011".into(),
        color12: "#000012".into(),
        color13: "#000013".into(),
        color14: "#000014".into(),
        color15: "#000015".into(),
        color16: None,
        color17: None,
        color18: None,
        color19: None,
        color20: None,
        color21: None,
        color22: None,
        color23: None,
    }
}

pub fn make_scheme() -> SchemeJson {
    SchemeJson {
        name: "test_scheme".into(),
        system: SchemeSystem::Base24,
        variant: SchemeVariant::Dark,
        palette: make_palette(),
        extra: Default::default(),
        tools: Default::default(),
    }
}
