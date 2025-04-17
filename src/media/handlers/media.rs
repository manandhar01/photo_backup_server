use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Extension, Json,
};
use std::{fs::create_dir_all, sync::Arc};

use crate::app::AppState;
use crate::media::{
    dtos::media::MediaResponse, models::media::Media, services::upload::UploadService,
};
use crate::user::models::user::User;

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<Vec<MediaResponse>>, (StatusCode, String)> {
    let user_dir = format!("./uploads/{}", user.uuid);
    create_dir_all(&user_dir).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not create upload dir".to_string(),
        )
    })?;

    let mut media_list: Vec<Media> = Vec::new();

    while let Some(field_result) = multipart.next_field().await.transpose() {
        let field =
            field_result.map_err(|_| (StatusCode::BAD_REQUEST, "Invalid field".to_string()))?;

        let media =
            UploadService::handle_upload_field(&state.db, &user_dir, user.id, field).await?;

        media_list.push(media);
    }

    if media_list.is_empty() {
        Err((StatusCode::BAD_REQUEST, "No files uploaded".to_string()))
    } else {
        let response: Vec<MediaResponse> = media_list.into_iter().map(Into::into).collect();
        Ok(Json(response))
    }
}
