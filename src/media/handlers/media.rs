use axum::{
    extract::{Multipart, Path, State},
    Extension, Json,
};
use std::sync::Arc;

use crate::errors::app_error::AppError;
use crate::media::{
    dtos::{media_list_payload::MediaListPayload, media_list_response::MediaListResponse},
    models::media::Media,
    services::{media::MediaService, media_metadata::MediaMetadataService, upload::UploadService},
};
use crate::user::{models::user::User, services::user::UserService};
use crate::{app::AppState, media::dtos::media_detail_response::MediaDetailResponse};

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<Vec<Media>>, AppError> {
    UserService::create_user_directory(&user).await?;

    let mut media_list: Vec<Media> = Vec::new();

    while let Some(field_result) = multipart.next_field().await.transpose() {
        let field = field_result.map_err(|_| AppError::BadRequest("Invalid field".to_string()))?;

        let media = UploadService::handle_upload_field(&state.db, &user, field).await?;

        media_list.push(media);
    }

    if media_list.is_empty() {
        Err(AppError::BadRequest("No files uploaded".to_string()))
    } else {
        Ok(Json(media_list))
    }
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
