// use axum::{
//     handler::{get, Handler},
//     http::{Request, Response, StatusCode},
//     response::{IntoResponse, Json},
//     routing::BoxRoute,
//     Router,
// };
// use std::convert::Infallible;



// pub async fn auth_middleware<S>(req: Request<Body>, service: S) -> Result<Response<Body>, Infallible>
// where
//     S: Service<Request<Body>, Response = Response<Body>, Error = Infallible> + Clone,
// {
//     // Check if the authorization token is present in the headers
//     if let Some(auth_header) = req.headers().get("authtoken") {
//         if let Ok(auth_token) = auth_header.to_str() {
//             // Here you can perform your authentication logic with the token
//             // For demonstration purposes, let's just check if it's "valid_token"
//             if auth_token == "valid_token" {
//                 // If the token is valid, call the inner service and return its response
//                 return Ok(service.clone().call(req).await.unwrap());
//             }
//         }
//     }
//     // If the token is not present or invalid, return a 403 Forbidden response
//     let response = Json(json!({
//         "error": "Unauthorized",
//     }))
//     .into_response();
//     Ok((response, StatusCode::FORBIDDEN))
// }
