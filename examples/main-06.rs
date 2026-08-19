use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::{Sleep, sleep};

#[tokio::main]
async fn main() {
    work().await
}

fn work() -> impl Future<Output = ()> {
    WorkFuture {
        delay_future: Box::pin(sleep(Duration::from_secs(1))),
    }
}

struct WorkFuture {
    delay_future: Pin<Box<Sleep>>,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Get a Pin<&mut impl Future<()>>
        // Requires some plumming with get_mut and as_mut because self is Pinned<&mut Self> and not &Self
        let pollable_delay_future = self.get_mut().delay_future.as_mut();

        // Poll the future manually
        let poll_result = pollable_delay_future.poll(cx);

        // If the delay is finished it's ready, so we do the next step
        // If not we keep waiting
        match poll_result {
            Poll::Ready(()) => {
                println!("Hello, world!");
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
