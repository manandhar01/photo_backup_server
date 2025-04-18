use axum::{extract::multipart::Field, http::StatusCode};
use std::{fs::OpenOptions, io::Write};

use crate::media::{
    enums::media_type::MediaType,
    models::media::Media,
    services::{extract::ExtractService, media::MediaService},
};
use crate::user::{models::user::User, services::user::UserService};

pub struct UploadService;

impl UploadService {
    pub async fn handle_upload_field(
        db: &sqlx::PgPool,
        user: &User,
        field: Field<'_>,
    ) -> Result<Media, (StatusCode, String)> {
        let original_filename = field
            .file_name()
            .map(|s| s.to_string())
            .ok_or((StatusCode::BAD_REQUEST, "Missing filename".to_string()))?;

        let filename = MediaService::sanitize_filename(&original_filename);

        let data = field
            .bytes()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid file data".to_string()))?
            .to_vec();

        UserService::create_user_directory(user).await?;
        let path = format!("uploads/{}/{}", user.uuid, filename);

        Self::save_file(&path, &data).await?;

        let attributes = ExtractService::extract_metadata(&path, &original_filename).await?;

        let media_type = match &attributes.mime_type {
            Some(m) => MediaType::from_mime(m) as i32,
            None => MediaType::Unknown as i32,
        };

        MediaService::create_media(db, user.id, &filename, &path, media_type, attributes)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))
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
}
