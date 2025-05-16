use axum::{extract::Multipart, Json};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use crate::errors::app_error::AppError;
use crate::media::{
    dtos::UploadResponseDto,
    enums::media_type_enum::MediaTypeEnum,
    models::media_metadata_model::MediaMetadataModel,
    services::{
        file_service::FileService, media_metadata_service::MediaMetadataService,
        media_service::MediaService, photo_service::PhotoService, video_service::VideoService,
    },
};
use crate::user::{models::user_model::UserModel, services::user_service::UserService};

pub struct UploadService {}

impl UploadService {
    pub async fn upload_chunk(
        db: &sqlx::PgPool,
        user: &UserModel,
        mut multipart: Multipart,
    ) -> Result<Json<UploadResponseDto>, AppError> {
        let mut file_name = String::new();
        let mut chunk_number = 0;
        let mut total_chunks = 0;
        let mut chunk_data = Vec::new();
        let mut original_file_name = String::new();

        while let Some(field) = match multipart.next_field().await {
            Ok(f) => f,
            Err(_) => {
                return Err(AppError::BadRequest("Something went wrong".into()));
            }
        } {
            let field_name = field.name().unwrap_or_default().to_string();

            match field_name.as_str() {
                "fileName" => {
                    original_file_name = field.text().await.unwrap_or_default();
                }
                "chunkNumber" => {
                    chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0);
                }
                "totalChunks" => {
                    total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0);
                }
                "chunk" => {
                    chunk_data = field
                        .bytes()
                        .await
                        .unwrap_or_else(|_| Vec::new().into())
                        .to_vec();
                }
                _ => {}
            }
        }

        if original_file_name.is_empty() || chunk_data.is_empty() {
            return Err(AppError::BadRequest(
                "Missing file name or chunk data".into(),
            ));
        }

        let temp_dir = format!("./uploads/{}/temp/{}", user.uuid, original_file_name);

        fs::create_dir_all(&temp_dir)
            .map_err(|_| AppError::InternalServerError("Failed to create temp dir".into()))?;

        let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
        let mut file = File::create(&chunk_path)
            .map_err(|_| AppError::InternalServerError("Failed to create chunk file".into()))?;
        file.write_all(&chunk_data)
            .map_err(|_| AppError::InternalServerError("Failed to write chunk".into()))?;

        if Self::is_upload_complete(&temp_dir, total_chunks) {
            file_name = FileService::sanitize_filename(&original_file_name);

            let final_path =
                Self::assemble_file(user, &file_name, &original_file_name, total_chunks).await?;

            let mime_type = infer::get_from_path(&final_path)
                .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
                .map(|t| t.mime_type().to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let media_type = MediaTypeEnum::from_mime(&mime_type) as i32;

            let media = MediaService::create_media(db, user, &file_name, &final_path, media_type)
                .await
                .map_err(|e| AppError::InternalServerError(format!("DB error: {}", e)))?;

            let metadata = Self::extract_metadata(&final_path, &original_file_name).await?;
            let _media_metadata =
                MediaMetadataService::create_metadata(db, &media, &metadata).await;

            if mime_type.starts_with("image/") {
                let _thumbnail =
                    PhotoService::generate_photo_thumbnail(&final_path, &file_name, 400, user)
                        .await;
            } else if mime_type.starts_with("video/") {
                let _thumbnail =
                    VideoService::generate_video_thumbnail(&final_path, &file_name, 400, user);
            }
        }

        Ok(Json(UploadResponseDto {
            success: true,
            message: "Chunk uploaded".into(),
            chunk_received: chunk_number,
            file_id: Some(file_name),
        }))
    }

    fn is_upload_complete(temp_dir: &str, total_chunks: usize) -> bool {
        match fs::read_dir(temp_dir) {
            Ok(entries) => entries.count() == total_chunks,
            Err(_) => false,
        }
    }

    async fn assemble_file(
        user: &UserModel,
        file_name: &str,
        original_file_name: &str,
        total_chunks: usize,
    ) -> Result<String, AppError> {
        UserService::create_user_directory(user).await?;

        let output_path = format!("./uploads/{}/{}", user.uuid, file_name);

        let mut output_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&output_path)
            .map_err(|_| (AppError::InternalServerError("Something went wrong".into())))?;

        let temp_dir = format!("./uploads/{}/temp/{}", user.uuid, original_file_name);

        for chunk_number in 0..total_chunks {
            let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);

            let chunk_data = fs::read(&chunk_path)
                .map_err(|_| (AppError::InternalServerError("Something went wrong".into())))?;
            output_file
                .write_all(&chunk_data)
                .map_err(|_| (AppError::InternalServerError("Something went wrong".into())))?;
        }

        fs::remove_dir_all(temp_dir)
            .map_err(|_| (AppError::InternalServerError("Something went wrong".into())))?;

        Ok(output_path)
    }

    async fn extract_metadata(
        filepath: &str,
        original_filename: &str,
    ) -> Result<MediaMetadataModel, AppError> {
        let mime_type = infer::get_from_path(filepath)
            .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let size = std::fs::metadata(filepath)
            .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
            .len() as i64;

        let mut metadata = MediaMetadataModel {
            mime_type: Some(mime_type.to_string()),
            size: Some(size),
            original_filename: Some(original_filename.to_string()),
            ..Default::default()
        };

        if mime_type.starts_with("image/") {
            PhotoService::extract_photo_metadata(filepath, &mut metadata);
        } else if mime_type.starts_with("video/") {
            VideoService::extract_video_metadata(filepath, &mut metadata);
        }

        if let Ok(hash) = FileService::generate_file_hash(filepath) {
            metadata.hash = Some(hash);
        }

        Ok(metadata)
    }
}
