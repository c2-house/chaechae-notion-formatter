use crate::error::NotionFormatterError;
use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String, NotionFormatterError> {
    fs::read_to_string(path)
        .map_err(|e| NotionFormatterError::FileReadError(format!("{}: {}", path.display(), e)))
}

pub fn create_dir_all(path: &Path) -> Result<(), NotionFormatterError> {
    fs::create_dir_all(path)
        .map_err(|e| NotionFormatterError::DirCreateError(format!("{}: {}", path.display(), e)))
}

pub fn write_file(path: &Path, content: &str) -> Result<(), NotionFormatterError> {
    fs::write(path, content)
        .map_err(|e| NotionFormatterError::FileWriteError(format!("{}: {}", path.display(), e)))
}

pub fn delete_file_and_dir(file_path: &Path, dir_path: &Path) -> Result<(), NotionFormatterError> {
    if file_path.exists() {
        fs::remove_file(file_path).map_err(|e| {
            NotionFormatterError::DeleteError(format!(
                "Failed to delete file {}: {}",
                file_path.display(),
                e
            ))
        })?;
    }
    if dir_path.exists() {
        fs::remove_dir_all(dir_path).map_err(|e| {
            NotionFormatterError::DeleteError(format!(
                "Failed to delete directory {}: {}",
                dir_path.display(),
                e
            ))
        })?;
    }
    Ok(())
}
