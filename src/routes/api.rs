use crate::controllers::{health, upload};
use crate::services::file_storage;
use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;

pub async fn testing() -> Json<serde_json::Value> {
    println!("This is just testing");

    file_storage::file_storage();

    Json(json!({"message": "This is a response"}))
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(testing))
        .route("/test", get(testing))
        .route("/health", get(health::health))
        .route("/upload", post(upload::upload_photo))
}
