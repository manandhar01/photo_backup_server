use chrono::Utc;

use crate::auth::services::auth_service::AuthService;
use crate::media::{
    dtos::{MediaListPayloadDto, MediaListResponseDto, PaginationMetadataDto},
    models::MediaModel,
};
use crate::user::models::UserModel;

pub struct MediaService {}

impl MediaService {
    pub async fn create_media(
        pool: &sqlx::PgPool,
        owner: &UserModel,
        filename: &str,
        filepath: &str,
        media_type: i32,
    ) -> Result<MediaModel, sqlx::Error> {
        let actor_id = AuthService::id();
        let now = Utc::now();

        let media = sqlx::query_as!(
            MediaModel,
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
        payload: MediaListPayloadDto,
    ) -> Result<MediaListResponseDto, sqlx::Error> {
        let limit = payload.limit.unwrap_or(20);
        let offset = payload.offset.unwrap_or(0);
        let user_id = AuthService::id();

        let media = sqlx::query_as!(
            MediaModel,
            r#"select * from media where deleted_at is null and user_id = $1 order by id desc limit $2 offset $3"#,
            user_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(r#"select count(*) from media where deleted_at is null"#)
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

        let response = MediaListResponseDto {
            data: media,
            pagination: PaginationMetadataDto {
                limit,
                offset,
                total,
            },
        };

        Ok(response)
    }

    pub async fn media_detail(pool: &sqlx::PgPool, id: i32) -> Result<MediaModel, sqlx::Error> {
        let media = sqlx::query_as!(
            MediaModel,
            r#"select * from media where deleted_at is null and id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(media)
    }

    pub async fn check_media_access(
        pool: &sqlx::PgPool,
        id: i32,
        user_id: i32,
    ) -> Result<MediaModel, sqlx::Error> {
        let media = sqlx::query_as!(
            MediaModel,
            r#"select * from media where deleted_at is null and id = $1 and user_id = $2"#,
            id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(media)
    }
}
