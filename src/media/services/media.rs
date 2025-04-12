use sqlx::types::Json;

use crate::media::models::media::{Media, MediaAttributes};

pub struct MediaService;

impl MediaService {
    pub async fn create_media(
        pool: &sqlx::PgPool,
        user_id: i32,
        filename: &str,
        filepath: &str,
        media_type: i32,
        mime_type: &str,
        size: u64,
    ) -> Result<Media, sqlx::Error> {
        let attributes = MediaAttributes {
            mime_type: Some(mime_type.to_string()),
            size: Some(size),
        };

        let media = sqlx::query_as!(
            Media,
            r#"
                insert into media (user_id, filename, filepath, media_type, attributes)
                values ($1, $2, $3, $4, $5)
                returning id, uuid, user_id, filename, filepath, media_type, created_at, updated_at, deleted_at, attributes as "attributes: Json<MediaAttributes>"
            "#,
            user_id,
            filename,
            filepath,
            media_type,
            serde_json::json!(attributes)
        )
        .fetch_one(pool)
        .await?;

        Ok(media)
    }
}
