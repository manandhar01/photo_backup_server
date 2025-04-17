use axum::{middleware, routing::get, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::middlewares::auth::auth_middleware;
use crate::user::handlers::user::{get_user_by_id, get_user_by_uuid};

pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/user/id/{id}", get(get_user_by_id))
        .route("/user/uuid/{uuid}", get(get_user_by_uuid))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
