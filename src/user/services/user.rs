use axum::http::StatusCode;
use chrono::Utc;
use sqlx::PgPool;
use std::fs::create_dir_all;
use std::path::Path;
use uuid::Uuid;

use crate::user::models::user::User;

pub struct UserService;

impl UserService {
    pub async fn create_user(
        pool: &sqlx::PgPool,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"insert into users (email, username, password) values ($1, $2, $3) returning *"#,
            email,
            username,
            password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"select * from users where email ilike $1 and deleted_at is null"#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_user_by_uuid(pool: &PgPool, uuid: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"select * from users where uuid = $1 and deleted_at is null"#,
            uuid
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_user_by_id(pool: &PgPool, user_id: i32) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"select * from users where id = $1 and deleted_at is null"#,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete_user(pool: &PgPool, user_id: i32) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"update users set deleted_at = $1 where id = $2 returning *"#,
            Utc::now(),
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create_user_directory(user: &User) -> Result<(), (StatusCode, String)> {
        let user_dir = format!("./uploads/{}", user.uuid);
        let path = Path::new(&user_dir);

        if !path.exists() {
            create_dir_all(&user_dir).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not create upload dir".to_string(),
                )
            })?;
        }

        Ok(())
    }
}
