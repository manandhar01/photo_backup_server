use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::{health, test, upload};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(test::test))
        .route("/test", get(test::test))
        .route("/health", get(health::health))
        .route("/upload", post(upload::upload_photo))
}
