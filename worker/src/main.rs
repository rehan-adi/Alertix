mod redis;
mod worker;

use worker::process_redis_queue;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    process_redis_queue().await;
}
