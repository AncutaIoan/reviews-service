use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

mod controller;
mod models;
mod router;
mod db;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");

    println!("🚀 Server running on http://127.0.0.1:8080");

    // Shared router across threads
    let router = Arc::new(router::Router::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let router = Arc::clone(&router);
                handle_client(stream, &router);
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream, router: &router::Router) {
    let mut buffer = [0; 1024]; // Buffer for reading incoming data
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer); // Convert buffer to string
            let response = router.handle_request(&request); // Handle request

            // Send back the response
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_) => {
            // Handle error reading the stream
            eprintln!("Error reading from stream.");
        }
    }
}