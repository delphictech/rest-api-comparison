// #![allow(unused)]

use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

mod handlers; // Import the handlers module

use handlers::my_function;

#[tokio::main]
async fn main() {
    // Define the routes
    let app = Router::new()
        .route("/", get(|| async { Html("HELLO WORLD") }))
        .route("/test", get(my_function()));

    // Define the address and port
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
