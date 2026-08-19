use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::runtime::Builder;

fn main() {
    let my_future = work();
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    runtime.block_on(my_future);
}

fn work() -> impl Future<Output = ()> {
    WorkFuture {}
}

struct WorkFuture {}

impl Future for WorkFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Hello, world!");
        Poll::Ready(())
    }
}
