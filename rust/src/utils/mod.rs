use std::collections::HashMap;

// Struct representing a user with authentication token and username
#[derive(Debug)]
pub struct User {
    pub auth_token: String,
    pub user_name: String,
}

// Struct representing coin details with balance
#[derive(Debug)]
pub struct CoinDetails {
    pub balance: u32,
}

// Function to retrieve mock login details
pub fn get_mock_login_details() -> HashMap<String, User> {
    let mut mock_login_details: HashMap<String, User> = HashMap::new();
    // Populate mock login details with sample user data
    mock_login_details.insert(
        String::from("alex"),
        User {
            auth_token: "123".to_string(),
            user_name: "alex".to_string(),
        },
    );
    mock_login_details.insert(
        String::from("jason"),
        User {
            auth_token: "234".to_string(),
            user_name: "jason".to_string(),
        },
    );
    mock_login_details.insert(
        String::from("marie"),
        User {
            auth_token: "345".to_string(),
            user_name: "marie".to_string(),
        },
    );
    mock_login_details
}

// Function to retrieve mock coin details
pub fn get_mock_coin_details() -> HashMap<String, CoinDetails> {
    let mut mock_coin_details: HashMap<String, CoinDetails> = HashMap::new();
    // Populate mock coin details with sample coin data
    mock_coin_details.insert(String::from("alex"), CoinDetails { balance: 100 });
    mock_coin_details.insert(String::from("jason"), CoinDetails { balance: 200 });
    mock_coin_details.insert(String::from("marie"), CoinDetails { balance: 300 });
    mock_coin_details
}
