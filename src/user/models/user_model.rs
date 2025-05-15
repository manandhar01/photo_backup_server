use bcrypt::verify;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: i32,
    pub uuid: Uuid,

    pub email: String,
    pub username: String,
    pub password: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

impl UserModel {
    pub fn verify_password(&self, password: &str) -> bool {
        match &self.password {
            Some(user_password) => verify(password, user_password).unwrap_or(false),
            None => false,
        }
    }
}

impl Clone for UserModel {
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
            created_by: self.created_by,
            updated_by: self.updated_by,
        }
    }
}
