use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum AyiouError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error), // Wrap std::io::Error

    #[error("Provider operation failed: {0}")]
    Provider(String), // Custom provider errors (e.g., S3, FTP)

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf), // Path-related errors

    #[error("Configuration error: {0}")]
    Config(String), // Config parsing issues

    #[error("Not found: {0}")]
    NotFound(String), // Resource not found
}
