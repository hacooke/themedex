pub mod models;
pub mod path;

use thiserror::Error;
use toml::de::Error;

#[derive(Error, Debug)]
pub enum ThemedexError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Toml(#[from] Error),
    #[error(transparent)]
    Template(#[from] minijinja::Error),
    #[error("Themedex internal error: {0}")]
    Internal(String),
    #[error("Unknown error")]
    Unknown,
}

pub fn internal_error(msg: &str) -> ThemedexError {
    ThemedexError::Internal(msg.to_string())
}

pub trait ToInternal<T> {
    fn internal_err(self, msg: &str) -> Result<T, ThemedexError>;
}

impl<T> ToInternal<T> for Option<T> {
    fn internal_err(self, msg: &str) -> Result<T, ThemedexError> {
        self.ok_or(ThemedexError::Internal(msg.to_string()))
    }
}

impl ThemedexError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Internal(msg) => msg.clone(),
            Self::Io(e) => e.to_string(),
            Self::Database(e) => e.to_string(),
            Self::Json(e) => e.to_string(),
            Self::Toml(e) => e.to_string(),
            Self::Template(e) => e.to_string(),
            Self::Unknown => "Unknown error".to_string(),
        }
    }
}

#[cfg(test)]
pub mod test_fixtures;
