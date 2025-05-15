use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponseDto {
    pub success: bool,
    pub message: String,
    pub chunk_received: usize,
    pub file_id: Option<String>,
}
