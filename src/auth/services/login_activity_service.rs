use sqlx::PgPool;

use crate::auth::{dtos::LoginActivityDto, models::LoginActivityModel};

pub struct LoginActivityService {}

impl LoginActivityService {
    pub async fn create_log(
        pool: &PgPool,
        activity: LoginActivityDto,
    ) -> Result<LoginActivityModel, sqlx::Error> {
        let record = sqlx::query_as!(
            LoginActivityModel,
            r#"insert into login_activity (user_id, email, success, ip_address, user_agent) values ($1, $2, $3, $4, $5) returning *"#,
            activity.user_id,
            activity.email,
            activity.success,
            activity.ip_address,
            activity.user_agent,
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }
}
