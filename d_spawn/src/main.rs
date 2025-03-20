#![allow(unused, dead_code)]
use log::Level;
use tokio::{io::AsyncReadExt, time};

async fn sleeper_a() {
    log::info!("a will sleep 5s");
    time::sleep(time::Duration::from_secs(5)).await;
    log::info!("a awake!");
}

async fn sleeper_b() {
    log::info!("b will sleep 3s");
    time::sleep(time::Duration::from_secs(3)).await;
    log::info!("b awake!");
}

async fn sleeper_c() {
    log::info!("c will sleep 1s");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("c awake!");
}

async fn run() {
    tokio::spawn(async {
        sleeper_a().await;
    });
    tokio::spawn(async {
        sleeper_c().await;
    });
    sleeper_b().await;
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("took {:?} secs.", end - start);
}
