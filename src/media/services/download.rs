use axum::body::Body;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

use crate::errors::app_error::AppError;
use crate::media::dtos::media_download_payload::MediaDownloadPayload;

pub struct DownloadService {}

impl DownloadService {
    pub async fn download_chunk(
        file_path: &str,
        payload: &MediaDownloadPayload,
    ) -> Result<(Body, usize), AppError> {
        let mut file = File::open(file_path)
            .await
            .map_err(|_| AppError::InternalServerError("Failed to open file".into()))?;

        file.seek(SeekFrom::Start(payload.offset))
            .await
            .map_err(|_| AppError::InternalServerError("Failed to seek file".into()))?;

        let mut buffer = vec![0u8; payload.chunk_size];
        let bytes_read = file
            .read(&mut buffer)
            .await
            .map_err(|_| AppError::InternalServerError("Failed to read file".into()))?;

        if bytes_read == 0 {
            return Err(AppError::EndOfFile);
        }

        let body = Body::from(buffer[..bytes_read].to_vec());

        Ok((body, bytes_read))
    }
}
