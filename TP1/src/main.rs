use std::io;
use std::collections::HashMap;

struct Compte {
    nom: String,
    solde: f32,
}


impl Compte {
    fn afficher_solde(&self) {
        println!("Solde de {}: {:.2}€", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f32) {
        if montant <= self.solde {
            self.solde -= montant;
            println!("Retrait de {:.2}€ effectué. Nouveau solde: {:.2}€", montant, self.solde);
        } else {
            println!("Fonds insuffisants !");
        }
    }
}

fn main() {
    let mut comptes: HashMap<String, Compte> = HashMap::new();

    // Création de quelques comptes de test
    comptes.insert("Kevin".to_string(), Compte { nom: "Kevin".to_string(), solde: 1500.0 });
    comptes.insert("Sophie".to_string(), Compte { nom: "Sophie".to_string(), solde: 2450.0 });

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    loop {
        println!("\n=== MENU ===");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("Entrez votre choix (1-{}):", options.len());

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrée invalide.");
                continue;
            }
        };

        if choix == 4 {
            println!("Merci, au revoir !");
            break;
        }

        match choix {
            1 => {
                let nom = demander_nom_utilisateur();
                if let Some(compte) = comptes.get(&nom) {
                    compte.afficher_solde();
                } else {
                    println!("Compte non trouvé !");
                }
            }
            2 => {
                let nom = demander_nom_utilisateur();
                if let Some(compte) = comptes.get_mut(&nom) {
                    println!("Montant à retirer:");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur lecture");
                    let montant: f32 = match montant.trim().parse() {
                        Ok(m) => m,
                        Err(_) => {
                            println!("Montant invalide !");
                            continue;
                        }
                    };
                    compte.retrait(montant);
                } else {
                    println!("Compte non trouvé !");
                }
            }
            3 => {
                println!("Liste des comptes:");
                for (nom, compte) in &comptes {
                    println!("- {} : {:.2}€", nom, compte.solde);
                }
            }
            _ => {
                println!("Choix invalide !");
            }
        }
    }
}


fn demander_nom_utilisateur() -> String {
    println!("Entrée le nom du titulaire de la carte :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur");
    nom.trim().to_string()
}