use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;

mod controller;
mod models;
mod router;
mod db;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind port");

    println!("🚀 Server running on http://127.0.0.1:8080");

    let router = Arc::new(router::Router::new());

    loop {
        let (mut stream, _) = listener.accept().await.expect("Failed to accept connection");

        let router = Arc::clone(&router);

        tokio::spawn(async move {
            handle_client(&mut stream, &router).await;
        });
    }
}

async fn handle_client(stream: &mut TcpStream, router: &router::Router) {
    let mut buffer = [0; 1024]; // Buffer for reading incoming data
    match stream.read(&mut buffer).await {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer); // Convert buffer to string
            let response = router.handle_request(&request); // Handle request

            // Send back the response
            stream.write_all(response.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        }
        Err(_) => {
            // Handle error reading from stream
            eprintln!("Error reading from stream.");
        }
    }
}