use serde::Serialize;

use crate::media::dtos::media_response::MediaResponse;

#[derive(Serialize)]
pub struct PaginationMetadata {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Serialize)]
pub struct MediaListResponse {
    pub data: Vec<MediaResponse>,
    pub pagination: PaginationMetadata,
}
