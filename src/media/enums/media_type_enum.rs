use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
pub enum MediaTypeEnum {
    Unknown = 0,
    Photo = 1,
    Video = 2,
}

impl From<i32> for MediaTypeEnum {
    fn from(media_type: i32) -> Self {
        match media_type {
            1 => MediaTypeEnum::Photo,
            2 => MediaTypeEnum::Video,
            _ => MediaTypeEnum::Unknown,
        }
    }
}

impl MediaTypeEnum {
    pub fn from_mime(mime: &str) -> MediaTypeEnum {
        if mime.starts_with("image/") {
            MediaTypeEnum::Photo
        } else if mime.starts_with("video/") {
            MediaTypeEnum::Video
        } else {
            MediaTypeEnum::Unknown
        }
    }
}
