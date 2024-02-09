use std::collections::HashMap;

use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use crate::utils::{get_mock_coin_details, CoinDetails};

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

pub async fn handler_coins_balance(
    Path(name): Path<String>,
) -> Result<axum::http::Response<Body>, StatusCode> {
    let coin_balance: HashMap<&'static str, CoinDetails> = get_mock_coin_details();

    println!("My object: {:?}", coin_balance["marie"].balance);

    // Create a JSON object with the desired message and code

    if let Some(value) = coin_balance.get("marie") {
        let json_response = serde_json::json!({
            "balance": value.balance,
            "userName": name
        });

        // Key found, do something with value
        Ok(Json(json_response).into_response())
    } else {
        // Key not found, return bad request error
        Err(StatusCode::BAD_REQUEST)
    }

    // Wrap the JSON object in the Json response type and return it
}
