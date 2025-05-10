use axum::{
    extract::{Multipart, Path, State},
    response::Response,
    Extension, Json,
};
use hyper::{HeaderMap, StatusCode};
use std::{io::SeekFrom, path::PathBuf, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

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

    let path = PathBuf::from(media.filepath);

    if !path.exists() {
        return Err(AppError::NotFound("File not found".into()));
    }

    let file_size = match metadata.size {
        Some(size) => size as u64,
        None => 0,
    };

    let range_header = headers.get("range").and_then(|h| h.to_str().ok());

    let (start, end) = if let Some(range) = range_header {
        // Example: bytes=1000-
        if let Some(range) = range.strip_prefix("bytes=") {
            let parts: Vec<_> = range.split('-').collect();
            let start = parts
                .first()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let end = parts
                .get(1)
                .and_then(|e| e.parse::<u64>().ok())
                .unwrap_or(file_size - 1);
            (start, end)
        } else {
            (0, file_size - 1)
        }
    } else {
        (0, file_size - 1)
    };
    let chunk_size = end - start + 1;

    let mut file = tokio::fs::File::open(&path).await.unwrap();
    file.seek(SeekFrom::Start(start)).await.unwrap();
    let stream = ReaderStream::with_capacity(file.take(chunk_size), 8192);

    let mime_type = metadata
        .mime_type
        .unwrap_or("application/octet-stream".to_string());

    let response = Response::builder()
        .status(StatusCode::PARTIAL_CONTENT)
        .header("Content-Type", mime_type)
        .header("Content-Length", chunk_size.to_string())
        .header("Accept-Ranges", "bytes")
        .header(
            "Content-Range",
            format!("bytes {}-{}/{}", start, end, file_size),
        )
        .body(axum::body::Body::from_stream(stream))
        .unwrap();

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
