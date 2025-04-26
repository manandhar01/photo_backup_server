use chrono::Utc;

use crate::auth::services::auth::AuthService;
use crate::media::models::media::Media;
use crate::user::models::user::User;

pub struct MediaService;

impl MediaService {
    pub async fn create_media(
        pool: &sqlx::PgPool,
        owner: &User,
        filename: &str,
        filepath: &str,
        media_type: i32,
    ) -> Result<Media, sqlx::Error> {
        let actor_id = AuthService::id();
        let now = Utc::now();

        let media = sqlx::query_as!(
            Media,
            r#"insert into media (user_id, filename, filepath, media_type, created_at, updated_at, created_by, updated_by) values ($1, $2, $3, $4, $5, $5, $6, $6) returning *"#,
            owner.id,
            filename,
            filepath,
            media_type,
            now,
            actor_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(media)
    }
}
