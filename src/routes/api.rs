use axum::Router;
use std::sync::Arc;

use crate::app::AppState;
use crate::routes::{auth::auth_routes, test::test_routes, user::user_routes};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(test_routes())
        .merge(auth_routes(app_state.clone()))
        .merge(user_routes(app_state.clone()))
}
