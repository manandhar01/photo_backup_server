use axum::{
    extract::{Multipart, Path, State},
    response::Response,
    Extension, Json,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::media::{
    dtos::{
        media_detail_response::MediaDetailResponse, media_list_payload::MediaListPayload,
        media_list_response::MediaListResponse,
    },
    services::{
        download::DownloadService,
        media::MediaService,
        media_metadata::MediaMetadataService,
        upload::{UploadResponse, UploadService},
    },
};
use crate::user::models::user::User;
use crate::{
    errors::app_error::AppError, media::dtos::media_download_payload::MediaDownloadPayload,
};

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    UploadService::upload_chunk(&state.db, &user, multipart).await
}

pub async fn download_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
    Json(payload): Json<MediaDownloadPayload>,
) -> Result<Response, AppError> {
    let media = MediaService::check_media_access(&state.db, id, user.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    let body = DownloadService::download_chunk(&media.filepath, payload).await?;

    let response = match Response::builder()
        .header("Content-Type", "application/octet-stream")
        .body(body)
    {
        Ok(res) => res,
        Err(_) => return Err(AppError::InternalServerError("Something went wrong".into())),
    };

    Ok(response)
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
