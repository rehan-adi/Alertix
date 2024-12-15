use std::env;
use dotenv::dotenv;
use actix_web::{App, HttpServer};

mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").expect("Failed to get PORT");
    println!("Server is running on {}", port);

    HttpServer::new(move || {
        App::new().service(routes::account::account)
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
