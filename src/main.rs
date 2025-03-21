// use crate::routes::api::create_routes;
// use std::net::SocketAddr;

mod controllers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // let app = create_routes();
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    //
    // println!("Server running at http://{}", addr);
    //
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    routes::api::testing();
    services::file_storage::file_storage();
    controllers::health::health();
    controllers::upload::upload_photo();
}
