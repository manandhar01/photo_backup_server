use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::media::{
    enums::media_type_enum::MediaTypeEnum,
    models::{media_metadata_model::MediaMetadataModel, media_model::MediaModel},
};

#[derive(Serialize)]
pub struct MediaDetailResponseDto {
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
    pub metadata: Option<MediaMetadataModel>,
}

impl From<(MediaModel, MediaMetadataModel)> for MediaDetailResponseDto {
    fn from((media, metadata): (MediaModel, MediaMetadataModel)) -> Self {
        Self {
            id: media.id,
            uuid: media.uuid,
            user_id: media.user_id,
            filename: media.filename,
            filepath: media.filepath,
            media_type: media.media_type,
            created_at: media.created_at,
            updated_at: media.updated_at,
            deleted_at: media.deleted_at,
            created_by: media.created_by,
            updated_by: media.updated_by,
            metadata: Some(metadata),
        }
    }
}
