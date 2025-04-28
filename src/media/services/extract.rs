use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::errors::app_error::AppError;
use crate::media::{
    models::media_metadata::MediaMetadata,
    services::{photo::PhotoService, video::VideoService},
};

pub struct ExtractService {}

impl ExtractService {
    pub async fn extract_metadata(
        filepath: &str,
        original_filename: &str,
    ) -> Result<MediaMetadata, AppError> {
        let mime_type = infer::get_from_path(filepath)
            .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let size = std::fs::metadata(filepath)
            .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?
            .len() as i64;

        let mut metadata = MediaMetadata {
            mime_type: Some(mime_type.to_string()),
            size: Some(size),
            original_filename: Some(original_filename.to_string()),
            ..Default::default()
        };

        if mime_type.starts_with("image/") {
            PhotoService::extract_photo_metadata(filepath, &mut metadata);
        } else if mime_type.starts_with("video/") {
            VideoService::extract_video_metadata(filepath, &mut metadata);
        }

        Self::generate_file_hash(filepath, &mut metadata);

        Ok(metadata)
    }

    fn generate_file_hash(path: &str, metadata: &mut MediaMetadata) {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return eprintln!("{:?}", e),
        };

        let mut bufreader = BufReader::new(file);
        let mut hasher = Sha256::new();

        let mut buffer = [0u8; 8192];
        loop {
            match bufreader.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    hasher.update(&buffer[..bytes_read]);
                }
                Err(e) => return eprintln!("{:?}", e),
            }
        }

        let result = hasher.finalize();

        metadata.hash = Some(format!("{:x}", result));
    }
}
