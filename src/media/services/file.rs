use axum::http::StatusCode;
use rand::{distr::Alphanumeric, Rng};
use std::{fs::OpenOptions, io::Write};

pub struct FileService {}

impl FileService {
    pub async fn save_file(path: &str, data: &[u8]) -> Result<(), (StatusCode, String)> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "File write error".to_string(),
                )
            })?;

        file.write_all(data).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "File write error".to_string(),
            )
        })
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
