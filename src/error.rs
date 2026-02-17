use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("{0}")]
    Other(String),
}

pub type MsResult<T> = Result<T, MsError>;

impl From<String> for MsError {
    fn from(s: String) -> Self {
        MsError::Other(s)
    }
}

impl From<&str> for MsError {
    fn from(s: &str) -> Self {
        MsError::Other(s.to_string())
    }
}

impl From<chrono::ParseError> for MsError {
    fn from(e: chrono::ParseError) -> Self {
        MsError::Parse(e.to_string())
    }
}

impl From<std::num::ParseIntError> for MsError {
    fn from(e: std::num::ParseIntError) -> Self {
        MsError::Parse(e.to_string())
    }
}

impl From<std::num::ParseFloatError> for MsError {
    fn from(e: std::num::ParseFloatError) -> Self {
        MsError::Parse(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for MsError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        MsError::Parse(e.to_string())
    }
}

impl From<url::ParseError> for MsError {
    fn from(e: url::ParseError) -> Self {
        MsError::Parse(e.to_string())
    }
}

impl From<toml::de::Error> for MsError {
    fn from(e: toml::de::Error) -> Self {
        MsError::Config(e.to_string())
    }
}

impl From<serde_yml::Error> for MsError {
    fn from(e: serde_yml::Error) -> Self {
        MsError::Parse(e.to_string())
    }
}
