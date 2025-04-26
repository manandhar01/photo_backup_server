use axum::{extract::multipart::Field, http::StatusCode};
use std::{fs::OpenOptions, io::Write};

use crate::media::{
    enums::media_type::MediaType,
    models::media::Media,
    services::{
        extract::ExtractService, media::MediaService, media_metadata::MediaMetadataService,
    },
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

        let mime_type = infer::get_from_path(&path)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let media_type = MediaType::from_mime(&mime_type) as i32;

        let media = MediaService::create_media(db, user, &filename, &path, media_type)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

        let metadata = ExtractService::extract_metadata(&path, &original_filename).await?;

        if let Ok(media_metadata) =
            MediaMetadataService::create_metadata(db, &media, &metadata).await
        {
            println!("{media_metadata:?}")
        }

        Ok(media)
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
