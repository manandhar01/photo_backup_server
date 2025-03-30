use axum::{
    // extract::Request,
    middleware,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::middlewares::auth::auth_middleware;
use crate::routes::auth::auth_routes;

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/protected", get(protected))
        .layer(middleware::from_fn(auth_middleware))
        .route("/", get(get_hello))
        .with_state(app_state.clone())
        .nest("/auth", auth_routes(app_state))
}

async fn protected() -> impl IntoResponse {
    "This is a protected resource"
}

async fn get_hello() -> impl IntoResponse {
    "Hello"
}
