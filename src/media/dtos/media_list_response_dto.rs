use serde::Serialize;

use crate::media::{dtos::PaginationMetadataDto, models::MediaModel};

#[derive(Serialize)]
pub struct MediaListResponseDto {
    pub data: Vec<MediaModel>,
    pub pagination: PaginationMetadataDto,
}
