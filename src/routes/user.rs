use axum::{middleware, routing::get, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::handlers::user::{get_user_by_id, get_user_by_uuid};
use crate::middlewares::auth::auth_middleware;

pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/user/id/{id}", get(get_user_by_id))
        .route("/user/uuid/{uuid}", get(get_user_by_uuid))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(app_state)
}
