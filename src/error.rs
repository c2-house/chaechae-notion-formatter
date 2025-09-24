use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotionFormatterError {
    // --- 기존에 사용하시던 에러 종류들 (그대로 유지) ---
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
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("URL decoding error")]
    UrlEncoding(#[from] FromUtf8Error),
    #[error("Image processing error")]
    Image(#[from] image::ImageError),
}
