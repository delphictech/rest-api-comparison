use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{Path, Request},
    http::{header, HeaderMap, Response, StatusCode},
    middleware::Next,
};

use crate::utils::{get_mock_login_details, User};

lazy_static::lazy_static! {
    // Retrieve mock login details lazily
    static ref LOGIN_DATA: HashMap<String, User> = get_mock_login_details();
}

// Authentication middleware function
pub async fn auth_middleware(
    Path(name): Path<String>, // Extract the user's name from the request path
    req: Request<Body>,       // Incoming request
    next: Next,               // Next middleware or handler in the chain
) -> Result<Response<Body>, StatusCode> {
    // Print a message to indicate that the middleware is fired
    println!("Middleware Fired!");

    // Get the path parameter as a String
    let name = name.to_string();

    // Get the headers from the request
    let headers: &HeaderMap = req.headers();

    // Get the value of the 'authtoken' header as a string
    let auth_token = headers
        .get(header::HeaderName::from_static("authtoken"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    // Clone login data into the async context
    let login_data = &*LOGIN_DATA;

    // Print the auth token for debugging purposes
    println!("User's auth token: {:?}", login_data["marie"].auth_token);

    // Check if the user exists in the login data and if the provided auth token matches
    if let Some(user) = login_data.get(&name) {
        if user.auth_token == auth_token {
            // If authentication is successful, proceed to the next middleware or handler
            return Ok(next.run(req).await);
        }
    }

    // If authentication fails, return a forbidden status code
    Err(StatusCode::FORBIDDEN)
}
