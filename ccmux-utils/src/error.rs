//! Common error types

/// Main error type for ccmux
#[derive(Debug, thiserror::Error)]
pub enum CcmuxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Connection error: {0}")]
    Connection(String),
}

/// Result type alias for ccmux
pub type Result<T> = std::result::Result<T, CcmuxError>;
