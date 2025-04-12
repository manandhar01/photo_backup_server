use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Extension, Json,
};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    sync::Arc,
};
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::dtos::claims::Claims;
use crate::media::dtos::media::MediaResponse;
use crate::media::enums::media_type::MediaType;
use crate::media::models::media::Media;
use crate::media::services::media::MediaService;
use crate::user::services::user::UserService;

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<Vec<MediaResponse>>, (StatusCode, String)> {
    let uuid = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Access denied".to_string()))?;

    let user = UserService::find_user_by_uuid(&state.db, uuid)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Access denied".to_string()))?;

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

        let filename = field
            .file_name()
            .map(sanitize_filename)
            .ok_or((StatusCode::BAD_REQUEST, "Missing filename".to_string()))?;

        let data = field
            .bytes()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid file data".to_string()))?;

        let mime_type = infer::get(&data)
            .map_or("application/octet-stream", |t| t.mime_type())
            .to_string();

        let size = data.len() as u64;
        let path = format!("{}/{}", user_dir, filename);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "File write error".to_string(),
                )
            })?;

        file.write_all(&data).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "File write error".to_string(),
            )
        })?;

        let media = MediaService::create_media(
            &state.db,
            user.id,
            &filename,
            &path,
            MediaType::from_mime(&mime_type) as i32,
            &mime_type,
            size,
        )
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

        media_list.push(media);
    }

    if media_list.is_empty() {
        Err((StatusCode::BAD_REQUEST, "No files uploaded".to_string()))
    } else {
        let response: Vec<MediaResponse> = media_list.into_iter().map(Into::into).collect();
        Ok(Json(response))
    }
}

fn sanitize_filename(filename: &str) -> String {
    let unsafe_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0'];

    let mut sanitized: String = filename
        .chars()
        .map(|c| {
            if unsafe_chars.contains(&c) || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect();

    sanitized = sanitized.trim().to_string();

    if sanitized.is_empty() {
        sanitized = "default_filename".to_string();
    }

    sanitized
}
