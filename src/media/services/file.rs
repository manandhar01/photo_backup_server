use rand::{distr::Alphanumeric, Rng};
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::errors::app_error::AppError;

pub struct FileService {}

impl FileService {
    pub async fn save_file(path: &str, data: &[u8]) -> Result<(), AppError> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::InternalServerError(format!("Failed to create directories: {}", e))
            })?;
        }

        let mut file = File::create(path)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create file: {}", e)))?;

        file.write_all(data)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to write file: {}", e)))
    }

    pub fn sanitize_filename(filename: &str) -> String {
        let unsafe_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0'];

        let mut sanitized: String = filename
            .chars()
            .map(|c| {
                if unsafe_chars.contains(&c) || c.is_control() {
                    '_'
                } else {
                    c
                }
            })
            .collect();

        sanitized = sanitized.trim().to_string();

        if sanitized.is_empty() {
            sanitized = "default_filename".to_string();
        }

        let prefix = Self::generate_random_prefix(8);

        format!("{}{}", prefix, sanitized)
    }

    fn generate_random_prefix(length: usize) -> String {
        let random_str: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        format!("{}_{}", random_str, chrono::Utc::now().timestamp_millis())
    }
}
