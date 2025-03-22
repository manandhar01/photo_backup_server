use axum::response::Json;
use serde_json::json;

pub async fn test() -> Json<serde_json::Value> {
    Json(json!({"message": "Test successful"}))
}
