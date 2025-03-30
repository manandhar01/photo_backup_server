use axum::Json;
use serde_json::{json, Value};

pub async fn test() -> Json<Value> {
    Json(json!({"message": "Test successful"}))
}
