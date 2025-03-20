#![allow(dead_code)]

use std::error::Error;

use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("waiting for ctrl-c");

    signal::ctrl_c().await?;

    println!("received ctrl-c signal");

    Ok(())
}
