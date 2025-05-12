use axum::{
    extract::{Multipart, Path, State},
    response::Response,
    Extension, Json,
};
use hyper::HeaderMap;
use std::sync::Arc;
use tokio::fs;

use crate::app::AppState;
use crate::errors::app_error::AppError;
use crate::media::{
    dtos::{
        media_detail_response::MediaDetailResponse, media_download_payload::MediaDownloadPayload,
        media_list_payload::MediaListPayload, media_list_response::MediaListResponse,
    },
    enums::media_type::MediaType,
    services::photo::PhotoService,
    services::{
        download::DownloadService,
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

pub async fn download_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
    Json(payload): Json<MediaDownloadPayload>,
) -> Result<Response, AppError> {
    let media = MediaService::check_media_access(&state.db, id, user.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    let (body, bytes_read) = DownloadService::download_chunk(&media.filepath, &payload).await?;

    let response = Response::builder()
        .header("Content-Type", "application/octet-stream")
        .header(
            "Content-Range",
            format!(
                "bytes {}-{}/?",
                payload.offset,
                payload.offset + bytes_read as u64 - 1
            ),
        )
        .body(body)
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    Ok(response)
}

pub async fn get_thumbnail(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let media = MediaService::check_media_access(&state.db, id, user.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    if media.media_type != MediaType::Photo {
        return Err(AppError::NotFound("Thumbnail not found".into()));
    }

    let mut thumbnail_path = format!("./uploads/{}/thumbnails/{}", user.uuid, media.filename);

    if !fs::try_exists(thumbnail_path.clone())
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?
    {
        thumbnail_path =
            PhotoService::generate_photo_thumbnail(&media.filepath, &media.filename, 400, &user)
                .await
                .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;
    }

    DownloadService::download_thumbnail(&thumbnail_path).await
}

pub async fn stream_media(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let media = MediaService::check_media_access(&state.db, id, user.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    let metadata = MediaMetadataService::get_metadata_for_media(&state.db, media.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    let response = DownloadService::stream_media(media, metadata, headers).await?;

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
