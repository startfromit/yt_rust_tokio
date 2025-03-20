#![allow(dead_code)]

use std::error::Error;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    timeout_demo().await;
    Ok(())
}

async fn timeout_demo() {
    if let Err(_) = time::timeout(time::Duration::from_secs(3), async {
        println!("sleeping");
        time::sleep(time::Duration::from_secs(2)).await;
        println!("slept");
    }).await {
        println!("closure timed out.");
    }
}

async fn interval_demo() {
    let duration = time::Duration::from_secs(1);
    let mut when = time::interval(duration);

    when.tick().await;
    println!("tick 1");
    when.tick().await;
    println!("tick 2");
    when.tick().await;
    println!("tick 3");
}
