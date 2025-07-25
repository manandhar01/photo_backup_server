use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::{
    handlers::{login, refresh_tokens, register, verify},
    middlewares::{auth_middleware, refresh_token_middleware},
};

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    let verify_route =
        Router::new()
            .route("/auth/verify", get(verify))
            .layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ));

    let refresh_tokens_route = Router::new()
        .route("/auth/refresh-tokens", post(refresh_tokens))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            refresh_token_middleware,
        ));

    let other_routes = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register));

    other_routes
        .merge(verify_route)
        .merge(refresh_tokens_route)
        .with_state(app_state)
}
