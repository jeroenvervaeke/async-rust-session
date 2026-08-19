use tokio::runtime::Builder;

fn main() {
    println!("building the future");
    let my_future = work();

    println!("building the runtime");
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    println!("calling block on future");
    runtime.block_on(my_future);
    println!("done blocking");
}

fn work() -> impl Future<Output = ()> {
    async {
        println!("Hello, world!");
    }
}
