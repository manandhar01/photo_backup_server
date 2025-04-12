use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    sync::Arc,
};
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::dtos::claims::Claims;
use crate::media::enums::media_type::MediaType;
use crate::media::services::media::MediaService;
use crate::user::services::user::UserService;

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return StatusCode::UNAUTHORIZED,
    };

    let user = match UserService::find_user_by_uuid(&state.db, uuid).await {
        Ok(user) => user,
        Err(_) => return StatusCode::UNAUTHORIZED,
    };

    let user_dir = format!("./uploads/{}", user.uuid);
    if let Err(e) = create_dir_all(&user_dir) {
        eprintln!("Failed to create upload dir: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    while let Some(field) = multipart.next_field().await.transpose() {
        let field = match field {
            Ok(f) => f,
            Err(_) => return StatusCode::BAD_REQUEST,
        };

        let filename = match field.file_name() {
            Some(name) => sanitize_filename(name),
            None => return StatusCode::BAD_REQUEST,
        };

        let data = match field.bytes().await {
            Ok(d) => d,
            Err(_) => return StatusCode::BAD_REQUEST,
        };

        let mime_type = infer::get(&data)
            .map_or("application/octet-stream", |t| t.mime_type())
            .to_string();

        let size = data.len() as u64;

        let path = format!("{}/{}", user_dir, filename);

        let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(f) => f,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };

        if file.write_all(&data).is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }

        let _media = MediaService::create_media(
            &state.db,
            user.id,
            &filename,
            &path,
            MediaType::from_mime(&mime_type) as i32,
            &mime_type,
            size,
        )
        .await;
    }

    StatusCode::CREATED
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
