use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use uuid::Uuid;

use crate::media::enums::media_type::MediaType;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaAttributes {
    pub mime_type: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
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
