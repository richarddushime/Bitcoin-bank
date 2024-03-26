mod database;
mod endpoints;
mod schema;

use crate::endpoints::{
    get_bank_balance, get_user_account_details_by_id, get_user_spending_history_by_id, get_users,spend_from_wallet,
    insert_user, insert_user_account_details, insert_user_spend, update_user_account_details,get_wallet_balance
};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use common::Wallet;
use env_logger::Env;
use actix_cors::Cors;
use actix_web::http::header;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

lazy_static::lazy_static! {
    static ref WALLET: Wallet = {

    let args: Vec<String> = std::env::args().collect();

    let wallet = Wallet::new(args[1].as_str());

    wallet
    };
}

#[actix_web::main]
 async fn main() -> std::io::Result<()> {
    dbg!(&WALLET.does_wallet_exist());
    println!(
        "INTIAL_WALLET_BALANCE: {}",
        WALLET.get_balance().unwrap().to_sat()
    );

    tokio::spawn(async {
        let duration = tokio::time::Duration::from_secs(30);
        let mut interval = tokio::time::interval(duration);

        loop {
            interval.tick().await;
            wallet_balancer(&WALLET).await;
        }
    });

    println!("Hello, world!");
    //let pool = database::init_pool();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .service(get_user_account_details_by_id)
            .service(get_user_spending_history_by_id)
            .service(insert_user)
            .service(insert_user_spend)
            .service(insert_user_account_details)
            .service(update_user_account_details)
            .service(get_users)
            .service(get_bank_balance)
            .service(get_wallet_balance)
            .service(spend_from_wallet)
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8090")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
    })
    .bind(("0.0.0.0", 3000))?
    .workers(2)
    .run()
    .await
}

async fn wallet_balancer(wallet: &Wallet) {
    if wallet.get_balance().unwrap().to_sat() < 100_000_000 {
        let mut stream = TcpStream::connect("127.0.0.1:8000")
            .await
            .expect("Failed to connect to server");

        let data_to_send = prepare_address(&wallet);
        stream
            .write_all(data_to_send.as_bytes())
            .await
            .expect("Failed to write data to stream");

        let mut received_data = String::new();
        stream
            .read_to_string(&mut received_data)
            .await
            .expect("Failed to read data from stream");
    }
}

fn prepare_address(wallet: &Wallet) -> String {
    wallet.get_p2wsh_address().unwrap()
}
