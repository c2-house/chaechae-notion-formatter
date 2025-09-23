use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotionFormatterError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Failed to read file: {0}")]
    FileReadError(String),
    #[error("Failed to write file: {0}")]
    FileWriteError(String),
    #[error("Failed to create directory: {0}")]
    DirCreateError(String),
    #[error("Failed to delete file or directory: {0}")]
    DeleteError(String),
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
}
