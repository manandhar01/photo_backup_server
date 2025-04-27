use chrono::Utc;
use sqlx::PgPool;

use crate::auth::services::auth::AuthService;
use crate::media::models::{media::Media, media_metadata::MediaMetadata};

pub struct MediaMetadataService {}

impl MediaMetadataService {
    pub async fn create_metadata(
        pool: &PgPool,
        media: &Media,
        metadata: &MediaMetadata,
    ) -> Result<MediaMetadata, sqlx::Error> {
        let actor_id = AuthService::id();
        let now = Utc::now();

        let row = sqlx::query_as!(
            MediaMetadata,
            r#"
                insert into media_metadata
                (media_id, original_filename, mime_type, size, width, height, hash, camera_make, camera_model, focal_length, aperture, taken_at, duration, frame_rate, video_codec, audio_codec, video_bitrate, audio_bitrate, sample_rate, created_at, updated_at, created_by, updated_by)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $20, $21, $21)
                returning *
            "#,
            media.id,
            metadata.original_filename,
            metadata.mime_type,
            metadata.size.map(|v| v as i64),
            metadata.width,
            metadata.height,
            metadata.hash,
            metadata.camera_make,
            metadata.camera_model,
            metadata.focal_length,
            metadata.aperture,
            metadata.taken_at,
            metadata.duration,
            metadata.frame_rate,
            metadata.video_codec,
            metadata.audio_codec,
            metadata.video_bitrate,
            metadata.audio_bitrate,
            metadata.sample_rate,
            now,
            actor_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn get_metadata_for_media(
        pool: &PgPool,
        media_id: i32,
    ) -> Result<MediaMetadata, sqlx::Error> {
        let metadata = sqlx::query_as!(
            MediaMetadata,
            r#"select * from media_metadata where deleted_at is null and media_id = $1"#,
            media_id
        )
        .fetch_one(pool)
        .await?;

        Ok(metadata)
    }
}
