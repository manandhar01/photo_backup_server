use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Json;
use uuid::Uuid;

use crate::media::{
    enums::media_type::MediaType,
    models::media::{Media, MediaAttributes},
};

#[derive(Debug, Serialize)]
pub struct MediaResponse {
    pub id: i32,
    pub uuid: Uuid,
    pub user_id: i32,
    pub filename: String,
    pub filepath: String,
    pub media_type: MediaType,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub attributes: Option<Json<MediaAttributes>>,
}

impl From<Media> for MediaResponse {
    fn from(media: Media) -> Self {
        MediaResponse {
            id: media.id,
            uuid: media.uuid,
            user_id: media.user_id,
            filename: media.filename,
            filepath: media.filepath,
            media_type: media.media_type,
            created_at: media.created_at,
            updated_at: media.updated_at,
            deleted_at: media.deleted_at,
            attributes: media.attributes,
        }
    }
}
