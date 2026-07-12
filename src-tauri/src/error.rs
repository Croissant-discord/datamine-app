use thiserror::Error;
use serde::{Serialize, Serializer};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Http Client error: {0}")]
    HttpClient(#[from] reqwest::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}