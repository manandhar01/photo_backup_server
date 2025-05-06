use axum::body::Body;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::{
    errors::app_error::AppError, media::dtos::media_download_payload::MediaDownloadPayload,
};

pub struct DownloadService {}

impl DownloadService {
    pub async fn download_chunk(
        file_path: &str,
        payload: MediaDownloadPayload,
    ) -> Result<Body, AppError> {
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(_) => return Err(AppError::InternalServerError("Something went wrong".into())),
        };

        let mut buffer = vec![0; payload.chunk_size];
        if file.seek(SeekFrom::Start(payload.offset)).is_err() {
            return Err(AppError::InternalServerError("Something went wrong".into()));
        }

        let bytes_read = match file.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return Err(AppError::InternalServerError("Something went wrong".into())),
        };

        if bytes_read == 0 {
            return Err(AppError::InternalServerError("Something went wrong".into()));
        }

        Ok(Body::from(buffer[..bytes_read].to_vec()))
    }
}
