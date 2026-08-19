use tokio::runtime::Builder;

fn main() {
    let my_future = work();

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

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
