use std::{
    fs::File,
    io::{BufReader, Read},
};

use rand::{distr::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

use crate::errors::app_error::AppError;

pub struct FileService {}

impl FileService {
    pub fn generate_file_hash(path: &str) -> Result<String, AppError> {
        let file = File::open(path)
            .map_err(|_| (AppError::InternalServerError("Failed to open file".into())))?;

        let mut bufreader = BufReader::new(file);
        let mut hasher = Sha256::new();

        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = bufreader
                .read(&mut buffer)
                .map_err(|_| (AppError::InternalServerError("Failed to generate hash".into())))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();

        Ok(format!("{:x}", result))
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
