// #![allow(unused)]

use axum::{http::Method, response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    let cors_middleware = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    // Define the routes
    let app = Router::new()
        .route("/", get(|| async { Html("HELLO WORLD") }))
        .route("/test", get(handlers::handler_test_real))
        .nest("/coins/:userid", {
            // Define routes specifically for /test
            Router::new().route("/", get(handlers::handler_coins_balance))
            // .layer(middleware::auth_middleware)
        })
        .layer(cors_middleware);

    // .route("/test2", get(handlers::handler_test));
    // .route("/coins/:name", get(handler_hello2).layer(auth_middleware));

    // Define the address and port
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
