use axum::{
    middleware,
    routing::{delete, get},
    Router,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::middlewares::auth_middleware::auth_middleware;
use crate::user::handlers::user_handler::{
    delete_user, get_self, get_user_by_id, get_user_by_uuid,
};

pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/user/self", get(get_self))
        .route("/user/id/{id}", get(get_user_by_id))
        .route("/user/uuid/{uuid}", get(get_user_by_uuid))
        .route("/user/id/{id}", delete(delete_user))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
