use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::user::models::user::User;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            uuid: user.uuid,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}
