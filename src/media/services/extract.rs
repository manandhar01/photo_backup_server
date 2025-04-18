use axum::http::StatusCode;

use crate::media::{
    models::media::MediaAttributes,
    services::{photo::PhotoService, video::VideoService},
};

pub struct ExtractService {}

impl ExtractService {
    pub async fn extract_metadata(
        filepath: &str,
        original_filename: &str,
    ) -> Result<MediaAttributes, (StatusCode, String)> {
        let mime_type = infer::get_from_path(filepath)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let size = std::fs::metadata(filepath)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .len();

        let mut attributes = MediaAttributes {
            mime_type: Some(mime_type.to_string()),
            size: Some(size),
            original_filename: Some(original_filename.to_string()),
            ..Default::default()
        };

        if mime_type.starts_with("image/") {
            PhotoService::extract_photo_metadata(filepath, &mut attributes)?;
        } else if mime_type.starts_with("video/") {
            VideoService::extract_video_metadata(filepath, &mut attributes)?;
        }

        Ok(attributes)
    }
}
