use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::{
    handlers::auth::{login, register, verify},
    middlewares::auth::auth_middleware,
};

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/verify", get(verify))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .with_state(app_state)
}
