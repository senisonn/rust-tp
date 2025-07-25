use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use std::{env, error::Error, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Lire le chemin du fichier depuis les arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <fichier_à_envoyer>", args[0]);
        return Ok(());
    }
    let filepath = &args[1];
    let path = Path::new(filepath);
    let filename = path.file_name().unwrap().to_str().unwrap();

    // Ouvre le fichier et lit tout son contenu
    let mut file = File::open(path).await?;
    let file_size = file.metadata().await?.len();

    // Connexion au serveur
    let mut stream = TcpStream::connect("127.0.0.1:4000").await?;
    println!("Connecté au serveur. Envoi de {} ({} octets)...", filename, file_size);

    // Envoie de l'en-tête
    stream.write_u8(0x01).await?; // opcode
    stream.write_u16(filename.len() as u16).await?;
    stream.write_all(filename.as_bytes()).await?;
    stream.write_u64(file_size).await?;

    // Envoie du contenu du fichier
    let mut buffer = vec![0u8; 4096];
    let mut remaining = file_size;
    while remaining > 0 {
        let read_size = buffer.len().min(remaining as usize);
        let n = file.read(&mut buffer[..read_size]).await?;
        if n == 0 { break; }
        stream.write_all(&buffer[..n]).await?;
        remaining -= n as u64;
    }

    println!("Fichier envoyé avec succès.");
    Ok(())
}
