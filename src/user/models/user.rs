use bcrypt::verify;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
