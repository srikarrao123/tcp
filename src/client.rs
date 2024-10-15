use tokio::net::{TcpSocket, TcpStream, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncBufRead, AsyncWriteExt, BufReader,AsyncReadExt, Stdin};
use core::error;
use std::error::Error;
use tokio::task::spawn_blocking;


async fn read_from_stdin(stdin: &mut Stdin) -> Result<String, std::io::Error> {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin);
    reader.read_line(&mut input).await?;
    Ok(input)
}

pub(crate) async fn run_client() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    println!("Connected to the chat server... Type your messages:");

    // Get a handle to stdin for reading
    let mut stdin = tokio::io::stdin();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result? == 0 {
                    println!("Server closed the connection.");
                    break;
                }
                println!("Received: {}", line);
                line.clear();
            }
            result = read_from_stdin(&mut stdin) => {
                let input = result?;
                if input.trim().is_empty() {
                    break;
                }
                writer.write_all(input.as_bytes()).await?;
            }
        }
    }
    Ok(())
}
