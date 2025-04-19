use axum::Json;
use serde_json::{json, Value};

use crate::media::services::extract::ExtractService;

pub async fn test() -> Json<Value> {
    let attributes = ExtractService::extract_metadata("./uploads/DSCN0012.jpg", "test.jpg").await;

    println!("{:#?}", attributes);

    Json(json!({"message": "Test successful"}))
}
