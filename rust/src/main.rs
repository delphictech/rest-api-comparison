use axum::{
    body::Body,
    extract::Path,
    http::{HeaderValue, Method, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, Response},
    routing::get,
    Router,
};

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod handlers;
mod middlewares;
mod utils;

async fn log_middleware(
    Path(name): Path<String>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Log information about the incoming request

    println!("Incoming request: {} {}", req.method(), req.uri().path());

    println!("{}", name.to_string());

    // Call the next handler in the chain
    let response = next.run(req).await;

    // Log information about the outgoing response
    println!("Outgoing response: {:?}", response.status());

    // Return the response
    Ok(response)
}

#[tokio::main]
async fn main() {
    let cors_middleware = CorsLayer::new()
        .allow_origin("http://localhost:8000/".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    // Define the routes
    let app = Router::new()
        .route("/", get(|| async { Html("HELLO WORLD") }))
        .route("/test", get(handlers::handler_test_real))
        .nest("/coins", {
            // Define routes specifically for /test
            Router::new()
                .route("/:userid", get(handlers::handler_coins_balance))
                .route_layer(middleware::from_fn(log_middleware))
            // .route_layer(middleware::from_fn(middlewares::auth_middleware))
        })
        .layer(cors_middleware);

    // .route("/test2", get(handlers::handler_test));
    // .route("/coins/:name", get(handler_hello2).layer(auth_middleware));

    // Define the address and port
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
