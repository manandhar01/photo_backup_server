use axum::{
    extract::{Multipart, Path, State},
    Extension, Json,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::errors::app_error::AppError;
use crate::media::{
    dtos::{
        media_detail_response::MediaDetailResponse, media_list_payload::MediaListPayload,
        media_list_response::MediaListResponse,
    },
    services::{
        media::MediaService,
        media_metadata::MediaMetadataService,
        upload::{UploadResponse, UploadService},
    },
};
use crate::user::models::user::User;

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    UploadService::upload_chunk(&state.db, &user, multipart).await
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
