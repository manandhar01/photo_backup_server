use serde::Serialize;

use crate::media::{
    dtos::pagination_metadat_dto::PaginationMetadataDto, models::media_model::MediaModel,
};

#[derive(Serialize)]
pub struct MediaListResponseDto {
    pub data: Vec<MediaModel>,
    pub pagination: PaginationMetadataDto,
}
