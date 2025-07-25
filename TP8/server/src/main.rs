use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    fs::File,
};
use std::{error::Error, path::PathBuf, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Serveur en écoute sur 127.0.0.1:4000");

    fs::create_dir_all("uploads")?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Connexion de {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("Erreur avec {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // Lire opcode (1 byte)
    let opcode = stream.read_u8().await?;
    if opcode != 0x01 {
        return Err("Opcode inconnu".into());
    }

    // Lire filename length (2 bytes)
    let name_len = stream.read_u16().await?;

    // Lire filename (variable)
    let mut name_buf = vec![0u8; name_len as usize];
    stream.read_exact(&mut name_buf).await?;
    let filename = String::from_utf8(name_buf)?;

    // Lire file size (8 bytes)
    let file_size = stream.read_u64().await?;
    println!("Fichier reçu: {} ({} octets)", filename, file_size);

    // Lire le contenu du fichier
    let mut file_path = PathBuf::from("uploads");
    file_path.push(&filename);
    let mut file = File::create(&file_path).await?;

    let mut remaining = file_size;
    let mut buffer = vec![0u8; 4096];

    while remaining > 0 {
        let read_size = buffer.len().min(remaining as usize);
        let n = stream.read_exact(&mut buffer[..read_size]).await?;
        file.write_all(&buffer[..n]).await?;
        remaining -= n as u64;
    }

    println!("Sauvegardé sous {}", file_path.display());
    Ok(())
}
