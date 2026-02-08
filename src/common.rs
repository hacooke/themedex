use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemedexError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
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
