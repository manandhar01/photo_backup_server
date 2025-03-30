use sqlx::PgPool;

use crate::{models::user::User, utils::hash::hash_password};

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
