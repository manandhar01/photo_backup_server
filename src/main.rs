use core::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

mod app;
mod auth;
mod config;
mod errors;
mod media;
mod test;
mod user;
mod utility;

use crate::app::create_app;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let app = create_app().await;

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
