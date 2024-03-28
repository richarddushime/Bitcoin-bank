use crate::endpoints::{
    get_bank_balance, get_user_account_details_by_id, get_user_spending_history_by_id, get_users,
    get_wallet_balance, insert_user, insert_user_account_details, insert_user_spend,
    spend_from_wallet, update_user_account_details,
};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use once_cell::sync::{Lazy, OnceCell};
use std::{io, process::Output};

mod database;
mod endpoints;
mod schema;

mod wallet;
pub use wallet::*;

mod btc_cli;
pub use btc_cli::*;

mod conf;
pub use conf::*;

mod data_dirs;
pub use data_dirs::*;

pub(crate) static COLD_WALLET_ADDRESS: OnceCell<String> = OnceCell::new();
pub(crate) static HOT_WALLET_ADDRESS: OnceCell<String> = OnceCell::new();
pub(crate) static HOT_CLIENT_RPC: OnceCell<RpcOps> = OnceCell::new();
pub(crate) static DATA_DIRS: Lazy<DataDirs> = Lazy::new(|| DataDirs::get_dirs());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load().unwrap();
    let config2 = config.clone();
    let config3 = config.clone();

    start_miner(&config);
    start_cold_wallet_node(&config);
    start_hot_wallet_node(&config);

    let cold_wallet_address = RpcOps::new(&config.cold).get_bech32m_address().unwrap();
    let hot_client_rpc = RpcOps::new(&config.hot);
    let hot_wallet_address = hot_client_rpc.get_bech32m_address().unwrap();
    COLD_WALLET_ADDRESS.get_or_init(|| cold_wallet_address);
    HOT_WALLET_ADDRESS.get_or_init(|| hot_wallet_address);
    HOT_CLIENT_RPC.get_or_init(|| hot_client_rpc);

    let client = RpcOps::new(&config.mining);
    if client.get_balance().unwrap().to_sat() < 100_000_000 {
        handle_output_err(
            BtcNative::bitcoin_cli(&config.mining).generate_blocks(100),
            &config,
        );
        handle_output_err(
            BtcNative::bitcoin_cli(&config.mining).generate_blocks(5),
            &config,
        );

        client
            .send_amount(COLD_WALLET_ADDRESS.get().unwrap(), 500_000_000)
            .unwrap();

        handle_output_err(
            BtcNative::bitcoin_cli(&config.mining).generate_blocks(5),
            &config,
        );
    }

    tokio::spawn(async move {
        mine_blocks(&config2).await;
    });

    tokio::spawn(async move {
        balancer(&config3).await;
    });

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
                    .max_age(3600),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .workers(2)
    .run()
    .await
}

pub fn start_miner(config: &Config) {
    handle_output_err(BtcNative::bitcoind_start(&config.mining, None), config);
    println!("WAITING FOR BITCOIND TO START");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("STARTED BITCOIND");
    handle_output_err(BtcNative::create_wallet(&config.mining), config);
}

pub async fn mine_blocks(config: &Config) {
    println!("MINING BLOCKS");

    let duration = tokio::time::Duration::from_secs(config.block_time as u64);
    let mut interval = tokio::time::interval(duration);

    let client = RpcOps::new(&config.mining);
    let mut balance;
    let mut send_amount;

    loop {
        interval.tick().await;
        handle_output_err(
            BtcNative::bitcoin_cli(&config.mining).generate_blocks(1),
            config,
        );

        balance = client.get_balance().unwrap().to_sat() as f64;
        send_amount = (balance * 0.95) as u64;
        dbg!(client.send_amount(COLD_WALLET_ADDRESS.get().unwrap(), send_amount));
    }
}

pub fn handle_output_err(value: io::Result<Output>, config: &Config) {
    match value {
        Ok(output) => println!("{:?}", output),
        Err(error) => stop_nodes(error, config).unwrap(),
    }
}

pub fn stop_nodes(error: impl std::error::Error, config: &Config) -> io::Result<()> {
    println!("Encountered Error: {}\n", error.to_string());
    println!(
        "STOPPING HOT WALLET BITCOIN DAEMON: {:?}\n",
        BtcNative::bitcoind_stop(&config.hot)?
    );
    println!(
        "STOPPING COLD WALLET BITCOIN DAEMON: {:?}\n",
        BtcNative::bitcoind_stop(&config.cold)?
    );
    println!(
        "STOPPING MINING NODE BITCOIN DAEMON: {:?}\n",
        BtcNative::bitcoind_stop(&config.mining)?
    );

    Ok(())
}

pub fn start_cold_wallet_node(config: &Config) {
    handle_output_err(
        BtcNative::bitcoind_start(&config.cold, Some(config.addnode.as_str())),
        config,
    );
    println!("WAITING FOR COLD WALLET BITCOIND TO START");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("STARTED BITCOIND");
    handle_output_err(BtcNative::create_wallet(&config.cold), config);
}

pub fn start_hot_wallet_node(config: &Config) {
    handle_output_err(
        BtcNative::bitcoind_start(&config.hot, Some(config.addnode.as_str())),
        config,
    );
    println!("WAITING FOR HOT WALLET BITCOIND TO START");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("STARTED BITCOIND");
    handle_output_err(BtcNative::create_wallet(&config.hot), config);
}

pub async fn balancer(config: &Config) {
    let hot_client = RpcOps::new(&config.hot);
    let cold_client = RpcOps::new(&config.cold);
    let duration = tokio::time::Duration::from_secs(8);
    let mut interval = tokio::time::interval(duration);

    loop {
        interval.tick().await;
        if hot_client.get_balance().unwrap().to_sat() < 100_000_000 {
            let current_cold_balance = cold_client.get_balance().unwrap().to_sat();
            let send_amount = (current_cold_balance as f64) * 0.45;
            dbg!(cold_client.send_amount(HOT_WALLET_ADDRESS.get().unwrap(), send_amount as u64));
        }
    }
}
