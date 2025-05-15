use serde::Serialize;

use crate::media::{dtos::pagination_metadat_dto::PaginationMetadataDto, models::media::Media};

#[derive(Serialize)]
pub struct MediaListResponseDto {
    pub data: Vec<Media>,
    pub pagination: PaginationMetadataDto,
}
