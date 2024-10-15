use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncBufReadExt, BufReader};
use tokio::sync::{ broadcast};
use tokio::sync::Mutex;
use std::error::Error;
use std::net::TcpStream;
use std::sync::Arc;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug)]
struct Message {
    content: String,
    sender: SocketAddr
}


pub(crate) async fn run_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is listening on port 8080");

    let (tx, _rx) = broadcast::channel::<Message>(10);

    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            // Client disconnected
                            break;
                        }
                        let message = Message {
                            content: line.clone(),
                            sender: addr,
                        };
                        if tx.send(message).is_err() {
                            // No listeners, message is dropped
                            break;
                        }
                        line.clear();
                    }
                    result = rx.recv() => {
                        if let Ok(message) = result {
                            if message.sender != addr {
                                if let Err(e) = writer.write_all(message.content.as_bytes()).await {
                                    eprintln!("Failed to send message to {}: {}", addr, e);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}