use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{Path, Request},
    http::{header, HeaderMap, Response, StatusCode},
    middleware::Next,
};

use crate::utils::{get_mock_login_details, User};

lazy_static::lazy_static! {
    static ref LOGIN_DATA: HashMap<String, User> = get_mock_login_details();
}

pub async fn auth_middleware(req: Request<Body>, next: Next, Path(name): Path<String>) -> Result<Response<Body>, StatusCode> {
    println!("Middleware Fired!");

    // get path/params
    let name = name.to_string();

    // Get the headers
    let headers: &HeaderMap = req.headers();

    // Get the auth token header value as a string
    let auth_token = headers
        .get(header::HeaderName::from_static("authtoken"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    // Clone login_data into the async context
    // let login_data = get_mock_login_details();

    let login_data = &*LOGIN_DATA;

    println!("My object: {:?}", login_data["marie"].auth_token);

    if let Some(user) = login_data.get(&name) {
        if user.auth_token == auth_token {
            // Insert the current user into a request extension so the handler can extract it
            // req.extensions_mut().insert(user.clone());
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
