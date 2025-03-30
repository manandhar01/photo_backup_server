use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

use crate::routes::api::create_routes;

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

    create_routes(app_state)
}
