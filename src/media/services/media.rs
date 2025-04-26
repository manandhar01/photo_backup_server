use rand::{distr::Alphanumeric, Rng};

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
        let user_id = AuthService::user().map(|u| u.id);

        let media = sqlx::query_as!(
            Media,
            r#"insert into media (user_id, filename, filepath, media_type, created_by, updated_by) values ($1, $2, $3, $4, $5, $6) returning *"#,
            owner.id,
            filename,
            filepath,
            media_type,
            user_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(media)
    }

    pub fn sanitize_filename(filename: &str) -> String {
        let unsafe_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0'];

        let mut sanitized: String = filename
            .chars()
            .map(|c| {
                if unsafe_chars.contains(&c) || c.is_control() {
                    '_'
                } else {
                    c
                }
            })
            .collect();

        sanitized = sanitized.trim().to_string();

        if sanitized.is_empty() {
            sanitized = "default_filename".to_string();
        }

        let prefix = Self::generate_random_prefix(8);

        format!("{}{}", prefix, sanitized)
    }

    fn generate_random_prefix(length: usize) -> String {
        let random_str: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        format!("{}_{}", random_str, chrono::Utc::now().timestamp_millis())
    }
}
