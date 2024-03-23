use actix_web::rt::time;
use config::Config;
use std::error::Error;
use rusqlite::Connection;
use std::thread;
use std::time::Duration;
use crate::{schema::{BankBalance, UserAccountDetails, UserSpendHistory, Users, Spend}, WALLET};




pub fn get_user_account_details(user_id: i32) -> Result<Option<Vec<UserAccountDetails>>, Box<dyn Error>> {
    let mut user_account_details_list : Vec<UserAccountDetails> = Vec::new();

    let connection = get_database_connection()?;
    let mut statement = connection.prepare("SELECT * FROM users_account_details WHERE user_id = ?1")?;
    let query_result = statement.query_map([&user_id], |row| {
        Ok(UserAccountDetails {
            user_id: row.get(1)?,
            balance: row.get(2)?,
            account_address: row.get(3)?
        })
    })?;

    for details in query_result {
        user_account_details_list.push(details.unwrap());
    }

    match user_account_details_list.len() > 0 {
        true => {
            Ok(Some(user_account_details_list))
        }
        false => {
            Ok(None)
        }
    }
}


pub fn get_bank_balance() -> Result<Option<Vec<BankBalance>>, Box<dyn Error>> {
    let mut balance_list : Vec<BankBalance> = Vec::new();

    let connection = get_database_connection()?;
    let mut statement = connection.prepare("SELECT * FROM bank_balance")?;
    //print!("{}", "in here");
    let query_result = statement.query_map([], |row| {
        Ok(BankBalance {
            id: row.get(0)?,
            hot_balance: row.get(1)?,
            cold_balance: row.get(2)?
        })
    })?;

    for balance in query_result {
        balance_list.push(balance?);
    }

    match balance_list.len() > 0 {
        true => {
            Ok(Some(balance_list))
        }
        false => {
            Ok(None)
        }
    }
}


pub fn get_user_spending_history(user_id: i32) -> Result<Option<Vec<UserSpendHistory>>, Box<dyn Error>> {
    let mut user_spending_history_list : Vec<UserSpendHistory> = Vec::new();

    let connection = get_database_connection()?;
    let mut statement = connection.prepare("SELECT * FROM users_spend_history WHERE user_id = ?1")?;
    let query_result = statement.query_map([&user_id], |row| {
        Ok(UserSpendHistory {
            user_id: row.get(1)?,
            source_address: row.get(2)?,
            destination_address: row.get(3)?,
            amount_spent: row.get(4)?,
            fees_amount: row.get(5)?
        })
    })?;

    for spending in query_result {
        user_spending_history_list.push(spending.unwrap());
    }

    match user_spending_history_list.len() > 0 {
        true => {
            Ok(Some(user_spending_history_list))
        }
        false => {
            Ok(None)
        }
    }
}

pub fn get_users() -> Result<Option<Vec<Users>>, Box<dyn Error>> {
    let mut user_list : Vec<Users> = Vec::new();

    let connection = get_database_connection()?;
    let mut statement = connection.prepare("SELECT * FROM users")?;
    //print!("{}", "in here");
    let query_result = statement.query_map([], |row| {
        Ok(Users {
            user_id: row.get(0)?,
            full_name: row.get(1)?,
            email: row.get(2)?,
            username: row.get(3)?,
            password: row.get(4)?,
            api_key: row.get(5)?
        })
    })?;

    for user in query_result {
        user_list.push(user?);
    }

    match user_list.len() > 0 {
        true => {
            Ok(Some(user_list))
        }
        false => {
            Ok(None)
        }
    }
}

pub fn insert_user(user: Users) -> Result<(), Box<dyn Error>> {
    let connection = get_database_connection()?;
    //let connection = pool.get().unwrap();
    //let new_user_id = Uuid::new_v4().simple().to_string();

   // thread::sleep(Duration::from_secs(30));
    print!("{}","here");
    let mut statement = connection.prepare("INSERT INTO users (full_name, username, email, password, api_key) VALUES (?, ?, ?,?,?)")?;
    let _ = statement.execute((user.full_name, user.username, user.email, user.password, user.api_key))?;

    Ok(())
}

pub fn insert_bank_balance(bank_balance: BankBalance) -> Result<(), Box<dyn Error>> {
    let connection = get_database_connection()?;
    //let connection = pool.get().unwrap();
    //let new_user_id = Uuid::new_v4().simple().to_string();

   // thread::sleep(Duration::from_secs(30));
    //print!("{}","here");
    let mut statement = connection.prepare("INSERT INTO bank_balance (hot_balance, cold_balance) VALUES (?,?)")?;
    let _ = statement.execute((bank_balance.hot_balance, bank_balance.cold_balance))?;

    Ok(())
}

pub fn insert_user_spend(user_spending: UserSpendHistory) -> Result<(), Box<dyn Error>> {
    let connection = get_database_connection()?;
    //let new_user_id = Uuid::new_v4().simple().to_string();

    let mut statement = connection.prepare("INSERT INTO users_spend_history (user_id, source_address, destination_address,amount_spent,fees_amount)
                                                         VALUES (?, ?, ?,?,?)")?;
    let _ = statement.execute((user_spending.user_id , user_spending.source_address , user_spending.destination_address, user_spending.amount_spent, user_spending.fees_amount))?;

    Ok(())
}

pub fn insert_user_account_details(user_account_details: UserAccountDetails) -> Result<(), Box<dyn Error>> {
    let connection = get_database_connection()?;
    //let new_user_id = Uuid::new_v4().simple().to_string();

    let mut statement = connection.prepare("INSERT INTO users_account_details (user_id, balance, account_address)
    VALUES (?, ?, ?)")?;
    let _ = statement.execute((user_account_details.user_id, user_account_details.balance, user_account_details.account_address))?;
    
    Ok(())
}

pub fn update_user_account_details(user_account_details: UserAccountDetails) -> Result<bool, Box<dyn Error>> {
    let connection = get_database_connection()?;
    //let new_user_id = Uuid::new_v4().simple().to_string();

    let mut statement = connection.prepare("UPDATE users_account_details SET balance = ?, account_address = ? WHERE user_id = ?")?;
    let result_count = statement.execute((user_account_details.user_id, user_account_details.balance, user_account_details.account_address))?;

    let mut succeeded =false;
    if result_count > 0 {
        succeeded = true;
    }
    
    Ok(succeeded)
}

pub fn get_database_connection() -> Result<Connection, Box<dyn Error>> {
    let settings = Config::builder().add_source(config::File::with_name("config")).build()?;
    let connection = Connection::open(settings.get_string("DATABASE_LOCATION")?)?;

    Ok(connection)
}



pub fn get_wallet_balance() -> Result<Option<f64>, Box<dyn Error>> {
    let my_wallet = WALLET.get_balance().unwrap().to_btc();
    //let wallet_client = my_wallet.client();
    //let wallet_balance_result = my_wallet.get_balance();
    
    Ok(Some(my_wallet))
}

pub fn spend_from_wallet(spend: Spend) -> Result<String, Box<dyn Error>> {
    let spend_address = spend.dest_address;
    let st_spend_address = spend_address.as_str();
    let transaction_id = WALLET.send_amount(st_spend_address, spend.amount).unwrap();
    Ok(transaction_id)
}
