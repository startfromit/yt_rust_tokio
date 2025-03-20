struct MyStruct {
    field: i32,
}

#[tokio::main]
async fn main() {
    let shared = std::sync::Arc::new(tokio::sync::Mutex::new(MyStruct { field: 0 }));

    let lock_a = shared.clone();
    let lock_b = shared.clone();

    let a = tokio::spawn(async move {
        let mut val = lock_a.lock().await;
        val.field += 1;
    });

    let b = tokio::spawn(async move {
        let mut val = lock_b.lock().await;
        val.field += 1;
    });

    tokio::join!(a, b).0.unwrap();

    let val = shared.lock().await;
    println!("val is {}", val.field);
}
