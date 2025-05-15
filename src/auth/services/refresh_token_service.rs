use axum::Json;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::auth::{
    dtos::login_response_dto::LoginResponseDto, models::refresh_token_model::RefreshTokenModel,
    services::auth_service::AuthService,
};
use crate::errors::app_error::AppError;
use crate::user::models::user_model::UserModel;

pub struct RefreshTokenService {}

impl RefreshTokenService {
    pub async fn generate_token_pair(
        pool: &PgPool,
        user: &UserModel,
        current_refresh_token: Option<String>,
        current_refresh_token_expires_at: Option<DateTime<Utc>>,
    ) -> Result<Json<LoginResponseDto>, AppError> {
        let access_token = AuthService::generate_access_token(user.id)
            .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

        let refresh_token = Self::generate_refresh_token(
            pool,
            user,
            current_refresh_token,
            current_refresh_token_expires_at,
        )
        .await?;

        Ok(Json(LoginResponseDto {
            access_token,
            refresh_token,
        }))
    }

    async fn generate_refresh_token(
        pool: &PgPool,
        user: &UserModel,
        current_refresh_token: Option<String>,
        current_refresh_token_expires_at: Option<DateTime<Utc>>,
    ) -> Result<String, AppError> {
        let new_refresh_token = AuthService::generate_refresh_token(user.id)
            .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

        if let (Some(token), Some(expires_at)) =
            (current_refresh_token, current_refresh_token_expires_at)
        {
            if Self::is_refresh_token_black_listed(pool, &token, user.id)
                .await
                .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?
            {
                return Err(AppError::Unauthorized("Invalid credentials".into()));
            }

            let _blacklisted = Self::black_list_refresh_token(pool, user.id, &token, expires_at)
                .await
                .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;
        }

        Ok(new_refresh_token)
    }

    async fn is_refresh_token_black_listed(
        pool: &PgPool,
        token: &str,
        user_id: i32,
    ) -> Result<bool, sqlx::Error> {
        let record = sqlx::query_as!(
            RefreshTokenModel,
            r#"select * from refresh_tokens where deleted_at is null and user_id = $1 and refresh_token = $2"#,
            user_id,
            token
        ).fetch_optional(pool).await?;

        Ok(record.is_some())
    }

    async fn black_list_refresh_token(
        pool: &PgPool,
        user_id: i32,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<RefreshTokenModel, sqlx::Error> {
        let record = sqlx::query_as!(
            RefreshTokenModel,
            r#"insert into refresh_tokens (user_id, refresh_token, expires_at) values ($1, $2, $3) returning *"#,
            user_id,
            token,
            expires_at
            )
            .fetch_one(pool)
            .await?;

        Ok(record)
    }
}
