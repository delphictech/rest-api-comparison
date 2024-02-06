use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub auth_token: &'static str,
    pub user_name: &'static str,
}

#[derive(Debug)]
pub struct CoinDetails {
    pub balance: u32,
}

pub fn get_mock_login_details() -> HashMap<&'static str, User> {
    let mut mock_login_details: HashMap<&str, User> = HashMap::new();
    mock_login_details.insert(
        "alex",
        User {
            auth_token: "123",
            user_name: "alex",
        },
    );
    mock_login_details.insert(
        "jason",
        User {
            auth_token: "234",
            user_name: "jason",
        },
    );
    mock_login_details.insert(
        "marie",
        User {
            auth_token: "345",
            user_name: "marie",
        },
    );
    mock_login_details
}

pub fn get_mock_coin_details() -> HashMap<&'static str, CoinDetails> {
    let mut mock_coin_details: HashMap<&str, CoinDetails> = HashMap::new();
    mock_coin_details.insert("alex", CoinDetails { balance: 100 });
    mock_coin_details.insert("jason", CoinDetails { balance: 200 });
    mock_coin_details.insert("marie", CoinDetails { balance: 300 });
    mock_coin_details
}
