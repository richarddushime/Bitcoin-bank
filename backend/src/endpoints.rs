use crate::schema::{Spend, UserAccountDetails, UserSpendHistory, Users};
use crate::{database, HOT_CLIENT_RPC};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use bitcoincore_rpc::Error as BtcError;
use web::{Json, Path};

#[get("bitcoinbank/balance/{user_id}")]
pub async fn get_user_account_details_by_id(id: Path<i32>) -> impl Responder {
    let user_id = id.into_inner();

    match database::get_user_account_details(user_id).unwrap() {
        Some(user_account_details_from_db) => HttpResponse::Ok()
            .content_type("application/json")
            .json(user_account_details_from_db),
        None => {
            HttpResponse::NotFound().body(format!("There is no user detail with id {}", user_id))
        }
    }
}

#[get("bitcoinbank/spending/{user_id}")]
pub async fn get_user_spending_history_by_id(id: Path<i32>) -> impl Responder {
    let user_id = id.into_inner();

    match database::get_user_spending_history(user_id.clone()).unwrap() {
        Some(user_spending_history_from_db) => HttpResponse::Ok()
            .content_type("application/json")
            .json(user_spending_history_from_db),
        None => HttpResponse::NotFound()
            .body(format!("There is no spending history with id {}", user_id)),
    }
}

#[get("bitcoinbank/users")]
pub async fn get_users() -> impl Responder {
    match database::get_users().unwrap() {
        Some(users_list) => HttpResponse::Ok()
            .content_type("application/json")
            .json(users_list),
        None => HttpResponse::NotFound().body(format!("There is no users")),
    }
}

#[get("bitcoinbank/bankbalance")]
pub async fn get_bank_balance() -> impl Responder {
    match database::get_bank_balance().unwrap() {
        Some(bank_balance_list) => HttpResponse::Ok()
            .content_type("application/json")
            .json(bank_balance_list),
        None => HttpResponse::NotFound().body(format!("There is no balance available")),
    }
}

#[post("bitcoinbank/signup")]
pub async fn insert_user(user: Json<Users>) -> impl Responder {
    match database::insert_user(user.into_inner()) {
        Ok(api_key) => HttpResponse::Ok().body(api_key),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("bitcoinbank/spend")]
pub async fn insert_user_spend(user_spending: Json<UserSpendHistory>) -> impl Responder {
    match database::insert_user_spend(user_spending.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("bitcoinbank/sendbalance")]
pub async fn insert_user_account_details(
    user_account_details: Json<UserAccountDetails>,
) -> impl Responder {
    match database::insert_user_account_details(user_account_details.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("bitcoinbank/balanceupdate")]
pub async fn update_user_account_details(
    user_account_details: Json<UserAccountDetails>,
) -> impl Responder {
    let updated_user_account_details = user_account_details.into_inner();
    match database::update_user_account_details(updated_user_account_details.clone()) {
        Ok(succeeded) => {
            if succeeded {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().body(format!(
                    "There is no user account detail with id {}",
                    updated_user_account_details.user_id
                ))
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("bitcoinbank/spendfromwallet")]
pub async fn spend_from_wallet(spend: Json<Spend>) -> impl Responder {
    match database::spend_from_wallet(HOT_CLIENT_RPC.get().unwrap(), spend.into_inner()).await {
        Ok(transaction_id) => HttpResponse::Ok()
            .content_type("application/json")
            .json(transaction_id),
        Err(error) => match error {
            BtcError::ReturnedError(error) => HttpResponse::Ok()
                .content_type("application/json")
                .json(error),
            BtcError::Json(error) => HttpResponse::Ok()
                .content_type("application/json")
                .json(error.to_string()),
            BtcError::JsonRpc(error) => HttpResponse::Ok()
                .content_type("application/json")
                .json(error.to_string()),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[get("bitcoinbank/getbalancefromwallet")]
pub async fn get_wallet_balance() -> impl Responder {
    match database::get_wallet_balance(HOT_CLIENT_RPC.get().unwrap()).unwrap() {
        Some(amount) => HttpResponse::Ok()
            .content_type("application/json")
            .json(amount),
        None => HttpResponse::NotFound().body(format!("There is no amount ")),
    }
}
