use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod app;
mod auth;
mod config;
mod dtos;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utils;

use crate::app::create_app;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = create_app().await;

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
