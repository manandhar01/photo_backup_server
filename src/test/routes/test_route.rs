use axum::{routing::get, Router};

use crate::test::handlers::test;

pub fn test_routes() -> Router {
    Router::new()
        .route("/", get(test))
        .route("/test", get(test))
}
