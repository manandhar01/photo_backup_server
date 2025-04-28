use axum::extract::multipart::Field;

use crate::errors::app_error::AppError;
use crate::media::{
    enums::media_type::MediaType,
    models::media::Media,
    services::{
        extract::ExtractService, file::FileService, media::MediaService,
        media_metadata::MediaMetadataService,
    },
};
use crate::user::{models::user::User, services::user::UserService};

pub struct UploadService;

impl UploadService {
    pub async fn handle_upload_field(
        db: &sqlx::PgPool,
        user: &User,
        field: Field<'_>,
    ) -> Result<Media, AppError> {
        let original_filename = field
            .file_name()
            .map(|s| s.to_string())
            .ok_or(AppError::BadRequest("Missing filename".to_string()))?;

        let filename = FileService::sanitize_filename(&original_filename);

        let data = field
            .bytes()
            .await
            .map_err(|_| AppError::BadRequest("Invalid file data".to_string()))?
            .to_vec();

        UserService::create_user_directory(user).await?;
        let path = format!("uploads/{}/{}", user.uuid, filename);

        FileService::save_file(&path, &data).await?;

        let mime_type = infer::get_from_path(&path)
            .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let media_type = MediaType::from_mime(&mime_type) as i32;

        let media = MediaService::create_media(db, user, &filename, &path, media_type)
            .await
            .map_err(|_| AppError::InternalServerError("DB error".to_string()))?;

        let metadata = ExtractService::extract_metadata(&path, &original_filename).await?;
        let _media_metadata = MediaMetadataService::create_metadata(db, &media, &metadata).await;

        Ok(media)
    }
}
