use axum::Json;
use serde_json::{json, Value};

use crate::media::services::extract::ExtractService;

pub async fn test() -> Json<Value> {
    let attributes =
        ExtractService::extract_metadata("./uploads/SampleVideo_1280x720_5mb.mp4", "test.mp4")
            .await;

    println!("{:#?}", attributes);

    Json(json!({"message": "Test successful"}))
}
