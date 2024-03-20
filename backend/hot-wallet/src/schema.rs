use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccountDetails{
    pub user_id: i32,
    pub balance: i32,
    pub account_address: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub api_key: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSpendHistory{
    pub user_id: i32,
    pub account_address: String,
    pub amount_spent: i32,
    pub destination_address: String,
    pub fees_amount: i32
}