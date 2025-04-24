use rand::{distr::Alphanumeric, Rng};

use crate::media::models::media::Media;

pub struct MediaService;

impl MediaService {
    pub async fn create_media(
        pool: &sqlx::PgPool,
        user_id: i32,
        filename: &str,
        filepath: &str,
        media_type: i32,
    ) -> Result<Media, sqlx::Error> {
        let media = sqlx::query_as!(
            Media,
            r#"insert into media (user_id, filename, filepath, media_type) values ($1, $2, $3, $4) returning *"#,
            user_id,
            filename,
            filepath,
            media_type,
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
