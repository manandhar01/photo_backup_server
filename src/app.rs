use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::auth::routes::auth_routes;
use crate::media::routes::media_routes;
use crate::test::routes::test_routes;
use crate::user::routes::user_routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn create_app() -> Router {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1)
        }
    };

    let app_state = Arc::new(AppState { db: pool.clone() });

    // let frontend_origin =
    //     std::env::var("FRONTEND_ORIGIN").unwrap_or("http://localhost:3000".to_string());
    // let allowed_origins = [frontend_origin.parse().unwrap()];
    //
    // let cors = CorsLayer::new()
    //     .allow_origin(allowed_origins)
    //     .allow_methods(Any)
    //     .allow_headers(Any);

    let cors = CorsLayer::very_permissive();

    Router::new()
        .merge(test_routes())
        .merge(auth_routes(app_state.clone()))
        .merge(user_routes(app_state.clone()))
        .merge(media_routes(app_state.clone()))
        .layer(cors)
}
