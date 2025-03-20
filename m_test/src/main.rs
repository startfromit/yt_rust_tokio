#![allow(dead_code)]

use std::error::Error;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn do_something() -> i32 {
    time::sleep(time::Duration::from_secs(1)).await;
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_something() {
        let res = do_something().await;
        assert_eq!(res, 5);
    }
}
