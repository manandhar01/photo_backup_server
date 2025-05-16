use serde::Serialize;

use crate::media::{dtos::PaginationMetadataDto, models::media_model::MediaModel};

#[derive(Serialize)]
pub struct MediaListResponseDto {
    pub data: Vec<MediaModel>,
    pub pagination: PaginationMetadataDto,
}
