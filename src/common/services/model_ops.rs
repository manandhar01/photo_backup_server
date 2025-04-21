use chrono::Utc;
use sqlx::PgPool;

pub struct ModelOpsService {}

impl ModelOpsService {
    pub async fn soft_delete(pool: &PgPool, table: &str, id: i32) -> sqlx::Result<()> {
        let query = format!("update {} set deleted_at = $1 where id = $2", table);

        sqlx::query(&query)
            .bind(Utc::now())
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
