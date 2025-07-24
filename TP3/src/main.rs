use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, Write, Read};
use std::path::Path;
use chrono::Utc;

struct Fichier {
    nom: String,
}

impl Fichier {
    fn creer(&self) -> io::Result<()> {
        let mut file = File::create(&self.nom)?;
        file.write_all(b"Bonjour a tous, fichier cree !")?;
        println!("Le fichier a ete cree avec succès.");
        Ok(())
    }

    fn ecrire(&self, contenu: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(&self.nom)?;
        writeln!(file, "{}", contenu)?;
        println!("Contenu ajoute au fichier.");
        Ok(())
    }

    fn lire(&self) -> io::Result<()> {
        let mut file = File::open(&self.nom)?;
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        println!("Contenu du fichier:\n{}", contenu);
        Ok(())
    }

    fn supprimer(&self) -> io::Result<()> {
        if Path::new(&self.nom).exists() {
            remove_file(&self.nom)?;
            println!("Fichier supprime.");
        } else {
            println!("Le fichier n'existe pas.");
        }
        Ok(())
    }

    fn date_creation(&self) -> io::Result<()> {
        let maintenant = Utc::now();
        println!("Date d'enregistrement (UTC) simulee : {}", maintenant);
        Ok(())
    }

}

fn main() -> io::Result<()> {
    let fichier = Fichier { nom: String::from("test.txt") };

    loop {
        println!("\n--- Menu ---");
        println!("1. Creer un fichier");
        println!("2. Lire le fichier");
        println!("3. ecrire dans le fichier");
        println!("4. Supprimer le fichier");
        println!("5. Voir la date de creation");
        println!("6. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix)?;
        let choix = choix.trim();

        match choix {
            "1" => fichier.creer()?,
            "2" => fichier.lire()?,
            "3" => {
                println!("Entrez le texte a ecrire :");
                let mut texte = String::new();
                io::stdin().read_line(&mut texte)?;
                fichier.ecrire(&texte.trim())?;
            }
            "4" => fichier.supprimer()?,
            "5" => fichier.date_creation()?,
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide, veuillez reessayer."),
        }
    }

    Ok(())
}
