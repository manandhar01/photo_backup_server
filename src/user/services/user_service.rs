use chrono::Utc;
use sqlx::PgPool;
use std::fs::create_dir_all;
use std::path::Path;
use uuid::Uuid;

use crate::auth::services::auth_service::AuthService;
use crate::errors::app_error::AppError;
use crate::user::models::UserModel;

pub struct UserService {}

impl UserService {
    pub async fn create_user(
        pool: &sqlx::PgPool,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<UserModel, sqlx::Error> {
        let actor_id = AuthService::id();
        let now = Utc::now();

        let user = sqlx::query_as!(
            UserModel,
            r#"insert into users (email, username, password, created_at, updated_at, created_by, updated_by) values ($1, $2, $3, $4, $4, $5, $5) returning *"#,
            email,
            username,
            password,
            now,
            actor_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<UserModel>> {
        sqlx::query_as!(
            UserModel,
            r#"select * from users where email ilike $1 and deleted_at is null"#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_user_by_uuid(pool: &PgPool, uuid: Uuid) -> sqlx::Result<Option<UserModel>> {
        sqlx::query_as!(
            UserModel,
            r#"select * from users where uuid = $1 and deleted_at is null"#,
            uuid
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_user_by_id(pool: &PgPool, user_id: i32) -> sqlx::Result<Option<UserModel>> {
        sqlx::query_as!(
            UserModel,
            r#"select * from users where id = $1 and deleted_at is null"#,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete_user(pool: &PgPool, user_id: i32) -> sqlx::Result<Option<UserModel>> {
        let actor_id = AuthService::id();
        let now = Utc::now();

        sqlx::query_as!(
            UserModel,
            r#"update users set deleted_at = $1, updated_at = $1, updated_by = $2 where id = $3 returning *"#,
            now,
            actor_id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create_user_directory(user: &UserModel) -> Result<(), AppError> {
        let user_dir = format!("./uploads/{}", user.uuid);
        let path = Path::new(&user_dir);

        if !path.exists() {
            create_dir_all(&user_dir).map_err(|_| {
                AppError::InternalServerError("Failed to create user directory".to_string())
            })?;
        }

        Ok(())
    }
}
