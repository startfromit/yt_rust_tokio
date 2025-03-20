use tokio::task;

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[tokio::main]
async fn main() {
    let a = task::spawn_blocking(|| {
        let result = fib(40);
        println!("fib(40) = {}", result);
    });

    let b = task::spawn_blocking(|| {
        let result = fib(39);
        println!("fib(39) = {}", result);
    });

    tokio::join!(a, b).0.unwrap();
}
