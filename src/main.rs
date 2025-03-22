use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod controllers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = routes::api::create_routes();
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
