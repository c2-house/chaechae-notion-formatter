use std::path::{Path, PathBuf};

use crate::{cli::Cli, error::NotionFormatterError};

#[derive(Debug)]
pub struct Config {
    pub source_file_path: PathBuf,
    pub source_dir_path: PathBuf,
    pub source_images_dir: PathBuf,
    pub slug: String,
    pub posts_dir: PathBuf,
    pub images_dir: PathBuf,
}

impl Config {
    pub fn new(cli: Cli) -> Result<Self, NotionFormatterError> {
        let source_file_path = PathBuf::from(cli.file_path);
        let source_dir_path = source_file_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();

        let slug = source_file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                NotionFormatterError::InvalidPath(source_file_path.display().to_string())
            })?
            .to_string();

        Ok(Config {
            source_file_path,
            source_dir_path: source_dir_path.clone(),
            source_images_dir: source_dir_path.join(&slug),
            slug,
            posts_dir: PathBuf::from(cli.posts_dir),
            images_dir: PathBuf::from(cli.images_dir),
        })
    }
}
