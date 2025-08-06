use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;

mod routes;
mod services;
mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(routes::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
