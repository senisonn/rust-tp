
// lecture à partir d'un fichier 
// pour lire un fichier on doit ouvrir le fichier et lire son contenu
// dans notre cas on on utilise Read et BufReader
// BufReader => crée un lecteur tamponé pour améliorer la performance

use std::fs::File; // stream  file system
use std::io::{self,BufReader,Read};
//use std::process::Command;

fn main() -> io::Result<()>{

       
        let file = File::open("test.txt")?;
        let mut reader = BufReader::new(file);// on crée un lecteur tamponné
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        println!("Ceci est le contenu du fichier {}", content);

      // Result::Ok(());
      // Result::Err("une erreur s'est produite!!!!!"); 

     // pour mettre en pause le terminal  lire.exe 
      let mut choix = String::new();
      let _= io::stdin().read_line(&mut choix);


    //  let _ = Command::new("pause").status(); // à vérifier ne fonctionne pas 

    Ok(())
}






/// ecriture de fichier 

use std::fs::File;
use std::io::{self, Write};

fn main() ->io::Result<()>  {


    let mut file = File::create("test.txt")?; // créer ou écraser un fichier
    file.write_all(b"Bonjour a  tous, fichier cree!" )?; // écrit des données dans le fichier
    // b est un byte string on utilise lorsque on travaille avec des données binaires 
    println!("le fichier a ete cree avec succes!!!");

      Ok(()) // signifie que que tout s'est bien passé et le () signifie rien à retourner
      Err(e) // signifie qu'une erreur i/o s'est produite  
     // io::Result<()>  retourne un résultat de type 

}
