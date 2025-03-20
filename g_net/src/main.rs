use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        let n = socket.read(&mut buf).await.unwrap();
        let data = std::str::from_utf8(&buf[0..n]).unwrap();
        socket
           .write_all(format!("Got your message {:?}, Dinosaur!\n", data).as_bytes())
           .await
           .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let host = "localhost:8080";
    let server = TcpListener::bind(host).await.unwrap();
    println!("server started");

    loop {
        let (socket, _) = server.accept().await.unwrap();
        tokio::spawn(handle_connection(socket));
    }
}
