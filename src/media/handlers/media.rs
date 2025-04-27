use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::media::{
    dtos::{
        media_list_payload::MediaListPayload, media_list_response::MediaListResponse,
        media_response::MediaResponse,
    },
    models::media::Media,
    services::{media::MediaService, upload::UploadService},
};
use crate::user::{models::user::User, services::user::UserService};

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<Vec<MediaResponse>>, (StatusCode, String)> {
    UserService::create_user_directory(&user).await?;

    let mut media_list: Vec<Media> = Vec::new();

    while let Some(field_result) = multipart.next_field().await.transpose() {
        let field =
            field_result.map_err(|_| (StatusCode::BAD_REQUEST, "Invalid field".to_string()))?;

        let media = UploadService::handle_upload_field(&state.db, &user, field).await?;

        media_list.push(media);
    }

    if media_list.is_empty() {
        Err((StatusCode::BAD_REQUEST, "No files uploaded".to_string()))
    } else {
        let response: Vec<MediaResponse> = media_list.into_iter().map(Into::into).collect();
        Ok(Json(response))
    }
}

pub async fn get_media_list(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MediaListPayload>,
) -> Result<Json<MediaListResponse>, (StatusCode, String)> {
    match MediaService::media_list(&state.db, payload).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )),
    }
}

pub async fn get_media_detail() {}
