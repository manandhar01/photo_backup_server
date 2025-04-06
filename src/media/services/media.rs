use crate::media::models::media::Media;

pub struct MediaService;

impl MediaService {
    pub async fn create_media(
        pool: &sqlx::PgPool,
        user_id: i32,
        filename: &str,
        filepath: &str,
        media_type: i32,
        mime_type: &str,
    ) -> Result<Media, sqlx::Error> {
        let media = sqlx::query_as!(
            Media,
            r#"
                insert into media (user_id, filename, filepath, media_type, mime_type)
                values ($1, $2, $3, $4, $5)
                returning id, uuid, user_id, filename, filepath, media_type, mime_type, created_at, updated_at, deleted_at
            "#,
            user_id,
            filename,
            filepath,
            media_type,
            mime_type
        )
        .fetch_one(pool)
        .await?;

        Ok(media)
    }
}
