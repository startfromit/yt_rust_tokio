use tokio::time;
use log::Level;

async fn run() {
    log::info!("will sleep 2s");
    time::sleep(time::Duration::from_secs(2)).await;
    log::info!("awake!");
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("took {:?} secs.", end - start);
}
