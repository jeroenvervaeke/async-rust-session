use std::{
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake},
    thread,
};

fn main() {
    let my_future = work();
    let mut runtime = Runtime {};
    runtime.block_on(my_future);
}

async fn work() {
    sub_task_1().await;
    sub_task_2().await;
}

async fn sub_task_1() {
    println!("in subtask 1");
}

async fn sub_task_2() {
    println!("in subtask 2");
}

struct Runtime {}

impl Runtime {
    pub fn block_on<F: Future<Output = ()>>(&mut self, fut: F) {
        // A smart runtime would queue the future and pick it up when it has capacity to do so
        // This is no such runtime, we'll pick up the work immediately and run the future till completion

        // Smart runtimes sleep and get signalled by a waker to do work
        // This one pass a waker that does nothing and ignores the waker signals
        let waker = Arc::new(NullWaker).into();
        let mut cx = Context::from_waker(&waker);

        // Pin the future in memory
        let mut pinned_future = pin!(fut);

        // Keep polling the future, asking if it's finished
        while let Poll::Pending = pinned_future.as_mut().poll(&mut cx) {
            // Yield the thread, so we're not running at 100% CPU
            thread::yield_now();
        }
    }
}

struct NullWaker;

impl Wake for NullWaker {
    fn wake(self: std::sync::Arc<Self>) {
        todo!()
    }
}
