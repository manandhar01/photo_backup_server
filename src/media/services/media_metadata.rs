use sqlx::PgPool;

use crate::media::models::{media::Media, media_metadata::MediaMetadata};

pub struct MediaMetadataService {}

impl MediaMetadataService {
    pub async fn create_metadata(
        pool: &PgPool,
        media: &Media,
        metadata: &MediaMetadata,
    ) -> Result<MediaMetadata, sqlx::Error> {
        let row = sqlx::query_as!(
            MediaMetadata,
            r#"
                insert into media_metadata
                (media_id, original_filename, mime_type, size, width, height, hash, camera_make, camera_model, focal_length, aperture, taken_at, duration, frame_rate, video_codec, audio_codec, video_bitrate, audio_bitrate, sample_rate)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
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
            metadata.sample_rate
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
