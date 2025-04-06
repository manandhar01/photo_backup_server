use axum::{middleware, routing::post, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::middlewares::auth::auth_middleware;
use crate::media::handlers::media::upload_chunk;

pub fn media_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/media", post(upload_chunk))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(app_state)
}
