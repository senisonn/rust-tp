use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use std::{sync::Arc, path::Path};
use chrono::Local;

const LOG_PATH: &str = "logs/server.log";

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    if !Path::new("logs").exists() {
        tokio::fs::create_dir("logs").await?;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .await?;

    let log_file = Arc::new(Mutex::new(file));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur en écoute sur 127.0.0.1:8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client connecté: {}", addr);

        let log_file = Arc::clone(&log_file);
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, log_file).await {
                eprintln!("Erreur client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, log_file: Arc<Mutex<tokio::fs::File>>) -> tokio::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_line = format!("[{}][{}] {}\n", timestamp, peer_addr, line);

        let mut file = log_file.lock().await;
        file.write_all(log_line.as_bytes()).await?;
        file.flush().await?;
    }

    println!("Client déconnecté: {}", peer_addr);
    Ok(())
}