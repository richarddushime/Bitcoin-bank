mod database;
mod schema;
mod endpoints;

use actix_web::{App, HttpServer};
use env_logger::Env;
use actix_web::middleware::Logger;

use crate::endpoints::{
    get_user_account_details_by_id, get_user_spending_history_by_id,
    insert_user, insert_user_spend,
    insert_user_account_details,update_user_account_details,get_users,get_bank_balance
};



#[actix_web::main]
async fn  main() -> std::io::Result<()>{
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
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 3000))?
        .workers(2)
        .run()
        .await
}
