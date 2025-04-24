use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct MediaMetadata {
    pub id: i32,
    pub uuid: Uuid,

    pub media_id: i32,
    pub original_filename: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub hash: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub focal_length: Option<String>,
    pub aperture: Option<String>,
    pub taken_at: Option<NaiveDateTime>,
    pub duration: Option<f64>,
    pub frame_rate: Option<f32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub video_bitrate: Option<String>,
    pub audio_bitrate: Option<String>,
    pub sample_rate: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}
