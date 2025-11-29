//! Error types for Miyabi

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiyabiError {
    #[error("Agent execution failed: {0}")]
    AgentError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, MiyabiError>;
