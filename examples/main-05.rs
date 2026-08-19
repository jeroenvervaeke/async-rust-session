use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    work().await
}

fn work() -> impl Future<Output = ()> {
    async {
        sleep(Duration::from_secs(1)).await;
        println!("Hello, world!");
    }
}
