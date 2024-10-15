
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use core::error;
use std::error::Error;

mod server;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Spawn server
    let server_handle = tokio::spawn(async {
        if let Err(e) = server::run_server().await {
            eprintln!("Server error: {}", e);
        }
        Ok::<(), Box<dyn Error + Send + Sync>>(()) // Ensure return is Ok(()) with the correct error type
    });

    // Delay to ensure the server starts before the client connects
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Spawn client
    let client_handle = tokio::spawn(async {
        if let Err(e) = client::run_client().await {
            eprintln!("Client error: {}", e);
        }
        Ok::<(), Box<dyn Error + Send + Sync>>(()) // Ensure return is Ok(()) with the correct error type
    });

    // Wait for both tasks (server and client) to complete
    tokio::try_join!(server_handle, client_handle)?;

    Ok(())
}
