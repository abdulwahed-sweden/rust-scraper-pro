use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(reqwest::StatusCode),
    
    #[error("Selector parsing error: {0}")]
    SelectorError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl From<toml::de::Error> for ScraperError {
    fn from(err: toml::de::Error) -> Self {
        ScraperError::ConfigError(err.to_string())
    }
}

impl From<csv::Error> for ScraperError {
    fn from(err: csv::Error) -> Self {
        ScraperError::ProcessingError(err.to_string())
    }
}

// Type alias for Results using ScraperError
pub type Result<T> = std::result::Result<T, ScraperError>;