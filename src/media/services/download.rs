use axum::{body::Body, response::Response};
use hyper::{HeaderMap, StatusCode};
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, SeekFrom},
};
use tokio_util::io::ReaderStream;

use crate::errors::app_error::AppError;
use crate::media::{
    dtos::media_download_payload_dto::MediaDownloadPayloadDto,
    models::{media_metadata_model::MediaMetadataModel, media_model::MediaModel},
};

pub struct DownloadService {}

impl DownloadService {
    pub async fn download_chunk(
        file_path: &str,
        payload: &MediaDownloadPayloadDto,
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

    pub async fn stream_media(
        media: MediaModel,
        metadata: MediaMetadataModel,
        headers: HeaderMap,
    ) -> Result<Response, AppError> {
        let path = PathBuf::from(media.filepath);

        if !path.exists() {
            return Err(AppError::NotFound("File not found".into()));
        }

        let mime_type = metadata
            .mime_type
            .unwrap_or("application/octet-stream".to_string());

        let file_size = match metadata.size {
            Some(size) => size as u64,
            None => 0,
        };

        let range_header = headers.get("range").and_then(|h| h.to_str().ok());

        let (start, end) = if let Some(range) = range_header {
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

    pub async fn download_thumbnail(file_path: &str) -> Result<Response, AppError> {
        let file = tokio::fs::File::open(&file_path)
            .await
            .map_err(|_| AppError::NotFound("File not found".into()))?;
        let stream = tokio_util::io::ReaderStream::new(file);

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "image/jpeg")
            .body(axum::body::Body::from_stream(stream))
            .map_err(|_| AppError::InternalServerError("Failed to build response".into()))?;

        Ok(response)
    }
}
