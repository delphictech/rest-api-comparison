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

pub async fn handler_coins_balance(Path(name): Path<String>) -> Result<Response<Body>, StatusCode> {
    let coin_balance: HashMap<String, CoinDetails> = get_mock_coin_details();

    println!("My object in route: {:?}", coin_balance["marie"].balance);

    // Convert the borrowed &str to String before using it as a key
    let name = name.to_string();

    if let Some(value) = coin_balance.get(&name) {
        let json_response = serde_json::json!({
            "balance": value.balance,
            "userName": name
        });

        Ok(Json(json_response).into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
