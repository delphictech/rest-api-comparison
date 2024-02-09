// use std::{collections::HashMap, path::Path};

// use axum::{
//     body::Body,
//     http::{self, StatusCode},
//     middleware::Next,
// };

// use crate::utils::{get_mock_login_details, User};

// pub async fn auth_middleware(
//     mut req: axum::http::Request<Body>,
//     next: Next,
// ) -> Result<axum::http::Response<Body>, StatusCode> {
//     const LOGIN_DATA: HashMap<&'static str, User> = get_mock_login_details();

//     let auth_header = req
//         .headers()
//         .get(http::header::AUTHORIZATION)
//         .and_then(|header| header.to_str().ok());

//     let auth_header = if let Some(auth_header) = auth_header {
//         auth_header
//     } else {
//         return Err(StatusCode::UNAUTHORIZED);
//     };

//     // Extract user ID from the path parameters
//     let user_id = match Path::extract(&mut req).await {
//         Ok(user_id) => user_id,
//         Err(_) => return Err(StatusCode::BAD_REQUEST),
//     };

//     if let Some(user) = LOGIN_DATA.get(&user_id) {
//         if user.auth_token == auth_header {
//             // Insert the current user into a request extension so the handler can extract it
//             req.extensions_mut().insert(user.clone());
//             return Ok(next.run(req).await);
//         }
//     }

//     Err(StatusCode::UNAUTHORIZED)
// }
