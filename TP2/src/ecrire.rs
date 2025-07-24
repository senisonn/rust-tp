/// ecriture de fichier 

use std::fs::File;
use std::io::{self, Write};

fn main() ->io::Result<()>  {
    let mut file = File::create("test.txt")?;
    file.write_all(b"Bonjour a  tous, fichier cree!" )?;
    println!("le fichier a ete cree avec succes!!!");

      Ok(()) 
      Err(e) 
}


fn createFile(fileName : String) -> File {
  File::create(fileName)?;
}
