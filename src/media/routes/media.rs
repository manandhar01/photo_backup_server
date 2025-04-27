use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::middlewares::auth::auth_middleware;
use crate::media::handlers::media::{get_media_detail, get_media_list, upload_chunk};

pub fn media_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/media", post(upload_chunk))
        .route("/media/list", post(get_media_list))
        .route("/media/{id}", get(get_media_detail))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
