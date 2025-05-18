use std::{thread, time::Instant};
use tokio::{runtime::Builder, task};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

async fn work(id: usize) {
    for step in 0..10 {
        let _ = fibonacci(40);
        println!("task {id} step {step} on {:?}", thread::current().id());

        // allow context switch to other tasks
        task::yield_now().await;
    }

    println!("task {id} done on {:?}", thread::current().id());
}

fn run_with(n: usize) {
    println!("\n===== worker_threads = {n} =====");

    let rt = Builder::new_multi_thread()
        .worker_threads(n)
        .enable_all()
        .build()
        .expect("failed to build runtime");

    let start = Instant::now();

    rt.block_on(async {
        let a = task::spawn(work(1));
        let b = task::spawn(work(2));
        let _ = tokio::join!(a, b);
    });

    println!("elapsed: {:.2?}", start.elapsed());
}

fn main() {
    run_with(1);

    run_with(2);
}
