use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::user::models::user_model::UserModel;

#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

impl From<UserModel> for UserResponseDto {
    fn from(user: UserModel) -> Self {
        UserResponseDto {
            id: user.id,
            uuid: user.uuid,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
            created_by: user.created_by,
            updated_by: user.updated_by,
        }
    }
}
