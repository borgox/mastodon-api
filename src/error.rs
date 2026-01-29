use thiserror::Error;

#[derive(Error, Debug)]
pub enum MastodonError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),

    #[error("JSON serialization/deserialization failed: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("API error (status {status}): {message}")]
    ApiError {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Rate limit exceeded. Retry after: {0}")]
    RateLimit(String),

    #[error("Custom error: {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, MastodonError>;
