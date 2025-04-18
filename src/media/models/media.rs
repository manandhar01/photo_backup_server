use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use uuid::Uuid;

use crate::media::enums::media_type::MediaType;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MediaAttributes {
    pub mime_type: Option<String>,
    pub size: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
    pub frame_rate: Option<f32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bitrate: Option<u64>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub focal_length: Option<f32>,
    pub iso: Option<u32>,
    pub aperture: Option<f32>,
    pub shutter_speed: Option<u16>,
    pub taken_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub hash: Option<String>,
    pub original_filename: Option<String>,
    pub color_profile: Option<String>,
    pub tags: Option<Vec<String>>,
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
