use axum::{routing::post, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::handlers::auth::{login, register};

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .with_state(app_state)
}
