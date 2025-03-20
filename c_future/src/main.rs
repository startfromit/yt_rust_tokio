#![allow(unused, dead_code)]
use log::Level;
use tokio::{io::AsyncReadExt, time};

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    log::info!("will sleep 2s");
    time::sleep(time::Duration::from_secs(2)).await;
    log::info!("awake!");
}

async fn reader() {
    log::info!("reading some nice emojis");
    let mut f = tokio::fs::File::open("emoji.i18n.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("read {} bytes", contents.len());

    tokio::task::spawn_blocking(|| {
        log::info!("computing fib(40)");
        fib(40);
        log::info!("done computing fib(40)");
    })
    .await
    .unwrap();
}

async fn run() {
    sleeper().await;
    sleeper().await;
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("took {:?} secs.", end - start);
}
