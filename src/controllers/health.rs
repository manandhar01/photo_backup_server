use axum::response::Json;
use serde_json::json;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({"message": "This is health function" }))
}
