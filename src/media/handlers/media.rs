use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    sync::Arc,
};

use crate::media::services::media::MediaService;
use crate::{app::AppState, media::enums::media_type::MediaType};

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or_default().to_string();
        let filename = sanitize_filename(field.file_name().unwrap());
        let data = &field.bytes().await.unwrap();
        let mime_type = infer::get(data).map_or("application/octet-stream", |t| t.mime_type());

        println!("Field Name: {}", name);
        println!("File Name: {}", filename);
        println!("MIME Type: {}", mime_type);
        println!("File Size: {}", data.len());

        let path = format!("./uploads/{}", filename);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();

        file.write_all(data).unwrap();
    }

    if true {
        let _media = MediaService::create_media(
            &state.db,
            1,
            "test",
            "path",
            MediaType::Photo as i32,
            "mimetype",
        )
        .await;
    }

    StatusCode::CREATED
}

fn is_upload_complete(temp_dir: &str, total_chunks: usize) -> bool {
    match fs::read_dir(temp_dir) {
        Ok(entries) => entries.count() == total_chunks,
        Err(_) => false,
    }
}

pub fn sanitize_filename(filename: &str) -> String {
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
