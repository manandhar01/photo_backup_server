use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginActivityModel {
    pub id: i32,
    pub user_id: Option<i32>,
    pub email: String,
    pub success: Option<bool>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
