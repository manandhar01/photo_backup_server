use axum::{
    extract::{Multipart, Path, State},
    Extension, Json,
};
use std::{fs::File, sync::Arc};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::{
    app::AppState,
    media::{dtos::media_detail_response::MediaDetailResponse, services::extract::ExtractService},
};
use crate::{errors::app_error::AppError, media::services::upload::UploadResponse};
use crate::{media::enums::media_type::MediaType, user::models::user::User};
use crate::{
    media::{
        dtos::{media_list_payload::MediaListPayload, media_list_response::MediaListResponse},
        services::{file::FileService, media::MediaService, media_metadata::MediaMetadataService},
    },
    user::services::user::UserService,
};

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
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

    if is_upload_complete(&temp_dir, total_chunks) {
        file_name = FileService::sanitize_filename(&original_file_name);

        let final_path =
            assemble_file(&user, &file_name, &original_file_name, total_chunks).await?;

        // Get MIME type
        let mime_type = infer::get_from_path(&final_path)
            .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let media_type = MediaType::from_mime(&mime_type) as i32;

        // Create media record
        let media =
            MediaService::create_media(&state.db, &user, &file_name, &final_path, media_type)
                .await
                .map_err(|e| AppError::InternalServerError(format!("DB error: {}", e)))?;

        // Extract and save metadata
        let metadata = ExtractService::extract_metadata(&final_path, &original_file_name).await?;
        let _media_metadata =
            MediaMetadataService::create_metadata(&state.db, &media, &metadata).await;
        //
        // Ok(media)
    }

    Ok(Json(UploadResponse {
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
    user: &User,
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

pub async fn get_media_list(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MediaListPayload>,
) -> Result<Json<MediaListResponse>, AppError> {
    match MediaService::media_list(&state.db, payload).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(AppError::InternalServerError(
            "Something went wrong".to_string(),
        )),
    }
}

pub async fn get_media_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    let media = MediaService::media_detail(&state.db, id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    let metadata = MediaMetadataService::get_metadata_for_media(&state.db, media.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    Ok(Json((media, metadata).into()))
}
