use std::path::Path;

use crate::common::{ThemedexError, internal_error};

pub struct PathComponents {
    pub directory: String,
    pub base_name: String,
    pub extensions: Vec<String>,
}

impl PathComponents {
    pub fn new(pathlike: impl AsRef<Path>) -> Result<Self, ThemedexError> {
        let path = pathlike.as_ref();
        let path_str = path
            .to_str()
            .ok_or_else(|| internal_error("Could not read path"))?;
        let dir = path.parent().and_then(|p| p.to_str()).ok_or_else(|| {
            ThemedexError::Internal(format!("Could not find parent for path {path_str}"))
        })?;
        let base_name = path.file_prefix().and_then(|p| p.to_str()).ok_or_else(|| {
            ThemedexError::Internal(format!("Could not find base file name for path {path_str}"))
        })?;
        let extensions: Vec<String> = path
            .file_name()
            .and_then(|path| path.to_str())
            .map(|name| name.strip_prefix(".").unwrap_or(name))
            .map(|norm_name| {
                norm_name
                    .split(".")
                    .skip(1)
                    .map(|ext| ext.to_owned())
                    .collect()
            })
            .unwrap_or_default();
        Ok(Self {
            directory: dir.to_owned(),
            base_name: base_name.to_owned(),
            extensions,
        })
    }
}

#[cfg(test)]
mod tests;
