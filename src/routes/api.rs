use crate::controllers::{health, upload};
use crate::services::file_storage;
use axum::{
    routing::{get, post},
    Router,
};

pub async fn testing() {
    println!("This is just testing");

    file_storage::file_storage();
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(testing))
        .route("/test", get(testing))
        .route("/health", get(health::health))
        .route("/upload", post(upload::upload_photo))
}
