use axum::http::StatusCode;
use sqlx::PgPool;
use std::fs::create_dir_all;
use std::path::Path;
use uuid::Uuid;

use crate::user::models::user::User;
use crate::utility::hash::hash_password;

pub struct UserService;

impl UserService {
    pub async fn create_user(
        pool: &sqlx::PgPool,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<User, sqlx::Error> {
        let hashed_password = hash_password(password);

        let user = sqlx::query_as!(
            User,
            r#"
        insert into users (email, username, password)
        values ($1, $2, $3)
        returning id, uuid, email, username, password, created_at, updated_at, deleted_at
        "#,
            email,
            username,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        select id, uuid, email, username, password, created_at, updated_at, deleted_at
        from users
        where email ilike $1
        "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        select id, uuid, email, username, password, created_at, updated_at, deleted_at
        from users
        where uuid = $1
        "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        select id, uuid, email, username, password, created_at, updated_at, deleted_at
        from users
        where id = $1
        "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
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
