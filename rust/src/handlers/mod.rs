use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

// pub async fn handler_test(Path(name): Path<String>) -> impl IntoResponse {
//     println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

//     Html(format!("Hello2 <strong>{name}</strong>"))
// }

pub async fn handler_test_real() -> Response {
    // Create a JSON object with the desired message and code
    let json_response = serde_json::json!({
        "message": "testing route",
        "code": 200,
    });

    // Wrap the JSON object in the Json response type and return it
    Json(json_response).into_response()
}
