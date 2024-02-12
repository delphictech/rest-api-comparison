use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub auth_token: String,
    pub user_name: String,
}

#[derive(Debug)]
pub struct CoinDetails {
    pub balance: u32,
}

pub fn get_mock_login_details() -> HashMap<String, User> {
    let mut mock_login_details: HashMap<String, User> = HashMap::new();
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

pub fn get_mock_coin_details() -> HashMap<String, CoinDetails> {
    let mut mock_coin_details: HashMap<String, CoinDetails> = HashMap::new();
    mock_coin_details.insert(String::from("alex"), CoinDetails { balance: 100 });
    mock_coin_details.insert(String::from("jason"), CoinDetails { balance: 200 });
    mock_coin_details.insert(String::from("marie"), CoinDetails { balance: 300 });
    mock_coin_details
}
