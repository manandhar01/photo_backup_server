use chrono::Utc;

use crate::auth::services::auth::AuthService;
use crate::media::{
    dtos::{
        media_list_payload::MediaListPayload,
        media_list_response::{MediaListResponse, PaginationMetadata},
        media_response::MediaResponse,
    },
    models::media::Media,
};
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

    pub async fn media_list(
        pool: &sqlx::PgPool,
        payload: MediaListPayload,
    ) -> Result<MediaListResponse, sqlx::Error> {
        let limit = payload.limit.unwrap_or(20);
        let offset = payload.offset.unwrap_or(0);

        let media = sqlx::query_as!(
            Media,
            r#"select * from media where deleted_at is null order by id desc limit $1 offset $2"#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(r#"select count(*) from media where deleted_at is null"#)
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

        let response = MediaListResponse {
            data: media.into_iter().map(MediaResponse::from).collect(),
            pagination: PaginationMetadata {
                limit,
                offset,
                total,
            },
        };

        Ok(response)
    }
}
