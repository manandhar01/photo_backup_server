use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
pub enum MediaType {
    Unknown = 0,
    Photo = 1,
    Video = 2,
}

impl From<i32> for MediaType {
    fn from(media_type: i32) -> Self {
        match media_type {
            1 => MediaType::Photo,
            2 => MediaType::Video,
            _ => MediaType::Unknown,
        }
    }
}
