use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    work().await
}

async fn work() {
    sleep(Duration::from_secs(1)).await;
    println!("Hello, world!");
}
