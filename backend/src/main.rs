use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod routes;
mod schema;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").expect("Failed to get PORT");
    println!("Server is running on {}", port);

    HttpServer::new(move || {
        App::new()
            .service(routes::loan::loan)
            .service(routes::account::account)
            .service(routes::transaction::transaction)
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
