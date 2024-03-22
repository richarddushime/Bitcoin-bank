use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccountDetails{
    pub user_id: i32,
    pub balance: f64,
    pub account_address: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Users{
    pub user_id: i32,
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub api_key: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSpendHistory{
    pub user_id: i32,
    pub source_address: String,
    pub amount_spent: f64,
    pub destination_address: String,
    pub fees_amount: f64
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankBalance{
    pub id: i32,
    pub hot_balance: f64,
    pub cold_balance: f64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spend{
    pub dest_address: String,
    pub amount: u64
}