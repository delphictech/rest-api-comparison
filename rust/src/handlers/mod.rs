use std::collections::HashMap;

use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use crate::utils::{get_mock_coin_details, CoinDetails};

// Handler for testing route
pub async fn handler_test_real() -> Response {
    // Create a JSON object with the desired message and code
    let json_response = serde_json::json!({
        "message": "testing route",
        "code": 200,
    });

    // Wrap the JSON object in the Json response type and return it
    Json(json_response).into_response()
}

// Handler for retrieving coin balance for a user
pub async fn handler_coins_balance(Path(name): Path<String>) -> Result<Response<Body>, StatusCode> {
    // Retrieve mock coin details
    let coin_balance: HashMap<String, CoinDetails> = get_mock_coin_details();

    // Print the balance for the user
    println!("User's balance: {:?}", coin_balance["marie"].balance);

    // Convert the borrowed &str to String before using it as a key
    let name = name.to_string();

    // Check if the user's balance exists in the mock data
    if let Some(value) = coin_balance.get(&name) {
        let json_response = serde_json::json!({
            "data": {
                "balance": value.balance,
                "userName": name
            }
        });

        // Return JSON response with the user's balance
        Ok(Json(json_response).into_response())
    } else {
        // If user's balance doesn't exist, return a bad request status code
        Err(StatusCode::BAD_REQUEST)
    }
}
