use axum::{extract::multipart::Field, http::StatusCode};
use rand::{distr::Alphanumeric, Rng};
use std::{fs::OpenOptions, io::Write};

use crate::media::{
    enums::media_type::MediaType, models::media::Media, services::media::MediaService,
};

pub struct UploadService;

impl UploadService {
    pub async fn handle_upload_field(
        db: &sqlx::PgPool,
        user_dir: &str,
        user_id: i32,
        field: Field<'_>,
    ) -> Result<Media, (StatusCode, String)> {
        let (filename, data, mime_type, size) = Self::extract_data(field).await?;
        let path = format!("{}/{}", user_dir, filename);

        Self::save_file(&path, &data).await?;

        MediaService::create_media(
            db,
            user_id,
            &filename,
            &path,
            MediaType::from_mime(&mime_type) as i32,
            &mime_type,
            size,
        )
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))
    }

    async fn extract_data(
        field: Field<'_>,
    ) -> Result<(String, Vec<u8>, String, u64), (StatusCode, String)> {
        let filename = field
            .file_name()
            .map(Self::sanitize_filename)
            .ok_or((StatusCode::BAD_REQUEST, "Missing filename".to_string()))?;

        let data = field
            .bytes()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid file data".to_string()))?
            .to_vec();

        let mime_type = infer::get(&data)
            .map_or("application/octet-stream", |t| t.mime_type())
            .to_string();

        let size = data.len() as u64;

        Ok((filename, data, mime_type, size))
    }

    async fn save_file(path: &str, data: &[u8]) -> Result<(), (StatusCode, String)> {
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

    fn sanitize_filename(filename: &str) -> String {
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
