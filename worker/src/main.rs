use db;
use worker::process_redis_queue;

mod redis;
mod worker;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    process_redis_queue().await;
}
