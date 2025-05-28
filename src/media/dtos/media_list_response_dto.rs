use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::media::{dtos::PaginationMetadataDto, enums::media_type_enum::MediaTypeEnum};

#[derive(Serialize)]
pub struct MediaListResponseDto {
    pub data: Vec<MediaListRow>,
    pub pagination: PaginationMetadataDto,
}

#[derive(Serialize)]
pub struct MediaListRow {
    pub id: i32,
    pub uuid: Uuid,

    pub user_id: i32,
    pub filename: String,
    pub filepath: String,
    pub media_type: MediaTypeEnum,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,

    pub media_metadata: Option<serde_json::Value>,
}
