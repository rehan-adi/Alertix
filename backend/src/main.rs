use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;

mod config;
mod models;
mod routes;
mod schema;
mod types;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();

    let pool = config::db::db_connect();

    let port = env::var("PORT").expect("Failed to get PORT");
    println!("Server is running on {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::loan::loan)
            .service(routes::account::account)
            .service(routes::transaction::transaction)
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
