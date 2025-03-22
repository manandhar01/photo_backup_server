use axum::response::Json;
use serde_json::json;

pub async fn upload_photo() -> Json<serde_json::Value> {
    Json(json!({"message":"function for uploading photo"}))
}
