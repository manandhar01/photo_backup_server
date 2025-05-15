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
use crate::media::services::video::VideoService;
use crate::media::{
    dtos::{
        media_detail_response_dto::MediaDetailResponseDto,
        media_download_payload_dto::MediaDownloadPayloadDto,
        media_list_payload_dto::MediaListPayloadDto, media_list_response_dto::MediaListResponseDto,
        upload_response_dto::UploadResponseDto,
    },
    enums::media_type_enum::MediaTypeEnum,
    services::photo::PhotoService,
    services::{
        download::DownloadService, media::MediaService, media_metadata::MediaMetadataService,
        upload::UploadService,
    },
};
use crate::user::models::user_model::UserModel;

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<UserModel>,
    multipart: Multipart,
) -> Result<Json<UploadResponseDto>, AppError> {
    UploadService::upload_chunk(&state.db, &user, multipart).await
}

pub async fn download_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<UserModel>,
    Path(id): Path<i32>,
    Json(payload): Json<MediaDownloadPayloadDto>,
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
    Extension(user): Extension<UserModel>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let media = MediaService::check_media_access(&state.db, id, user.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    if media.media_type != MediaTypeEnum::Photo && media.media_type != MediaTypeEnum::Video {
        return Err(AppError::NotFound("Thumbnail not found".into()));
    }

    let mut thumbnail_path = String::new();

    if media.media_type == MediaTypeEnum::Photo {
        thumbnail_path = format!("./uploads/{}/thumbnails/{}", user.uuid, media.filename);
    } else if media.media_type == MediaTypeEnum::Video {
        let stem = std::path::Path::new(&media.filename)
            .file_stem() // gets the filename without extension
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")
            .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

        thumbnail_path = format!("./uploads/{}/thumbnails/{}.webp", user.uuid, stem);
    }

    if !fs::try_exists(thumbnail_path.clone())
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?
    {
        if media.media_type == MediaTypeEnum::Photo {
            thumbnail_path = PhotoService::generate_photo_thumbnail(
                &media.filepath,
                &media.filename,
                400,
                &user,
            )
            .await
            .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;
        } else if media.media_type == MediaTypeEnum::Video {
            thumbnail_path = VideoService::generate_video_thumbnail(
                &media.filepath,
                &media.filename,
                400,
                &user,
            )
            .await
            .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;
        }
    }

    DownloadService::download_thumbnail(&thumbnail_path).await
}

pub async fn stream_media(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<UserModel>,
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
    Json(payload): Json<MediaListPayloadDto>,
) -> Result<Json<MediaListResponseDto>, AppError> {
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
) -> Result<Json<MediaDetailResponseDto>, AppError> {
    let media = MediaService::media_detail(&state.db, id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    let metadata = MediaMetadataService::get_metadata_for_media(&state.db, media.id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    Ok(Json((media, metadata).into()))
}
