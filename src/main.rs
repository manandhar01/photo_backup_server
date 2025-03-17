use axum::{body::Body, response::Json, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // Get the port number from the environment, default to 3000
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    // Run the app with hyper, listening on the specified address
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello world, from Axum!"
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "data": "This is a foo function" }))
}

async fn post_foo() -> &'static str {
    "This is post foo"
}

async fn foo_bar() -> &'static str {
    "This is foo bar"
}
