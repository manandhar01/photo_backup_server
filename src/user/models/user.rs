use bcrypt::verify;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::common::{services::model_ops::ModelOpsService, traits::model_ops::ModelOps};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap_or(false)
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            uuid: self.uuid,
            email: self.email.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

impl ModelOps for User {
    // async fn find() -> sqlx::Result<Option<Self>> {}
    //
    // async fn create(&self) -> sqlx::Result<Self> {}

    async fn soft_delete(&mut self, pool: &PgPool) -> sqlx::Result<()> {
        ModelOpsService::soft_delete(pool, "users", self.id).await?;
        self.deleted_at = Some(Utc::now());
        Ok(())
    }
}
