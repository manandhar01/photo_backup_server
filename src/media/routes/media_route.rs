use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::middlewares::auth_middleware;
use crate::media::handlers::{
    download_chunk, get_media_detail, get_media_list, get_thumbnail, stream_media, upload_chunk,
};

pub fn media_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/media", post(upload_chunk))
        .route("/media/list", post(get_media_list))
        .route("/media/{id}/download", post(download_chunk))
        .route("/media/{id}/thumbnail", get(get_thumbnail))
        .route("/media/{id}/stream", get(stream_media))
        .route("/media/{id}", get(get_media_detail))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
