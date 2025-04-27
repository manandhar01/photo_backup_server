use serde::Serialize;

use crate::media::models::media::Media;

#[derive(Serialize)]
pub struct PaginationMetadata {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Serialize)]
pub struct MediaListResponse {
    pub data: Vec<Media>,
    pub pagination: PaginationMetadata,
}
