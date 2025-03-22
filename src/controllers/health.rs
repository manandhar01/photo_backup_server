use axum::response::Json;
use serde_json::json;

use crate::services::file_storage;

pub async fn health() -> Json<serde_json::Value> {
    file_storage::file_storage();

    Json(json!({"message": "This is health function" }))
}
