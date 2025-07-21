🏦 TP1 – Gestion de Comptes Bancaires en Rust

Ce projet est un petit programme en ligne de commande, écrit en Rust, permettant de gérer des comptes bancaires à travers un menu interactif.
📋 Fonctionnalités

    ✅ Affichage du solde d’un compte

    ✅ Retrait d’un montant

    ✅ Affichage de la liste des comptes

    ✅ Menu interactif avec saisie utilisateur

🚀 Lancement du projet

# Clone ou place-toi dans le dossier TP1
cd TP1

# Compile et exécute le projet
cargo run

🧑‍💻 Exemple d'utilisation

=== MENU ===
1. Afficher solde
2. Retrait
3. Liste comptes
4. Quitter
👉 Entrez le numéro de l'action à effectuer (1-4):
1
Entrez le nom du titulaire du compte:
Kevin
Solde de Kevin: 1500.00€

💡 Comptes disponibles au démarrage

Le programme contient par défaut deux comptes bancaires :
Nom	Solde
Kevin	1500.00€
Sophie	2450.00€
📁 Structure

Le code est dans src/main.rs et utilise :

    struct Compte : modélise un compte avec un nom et un solde.

    HashMap : pour stocker dynamiquement plusieurs comptes.

    Fonctions :

        demander_nom_utilisateur() : demande le nom d’un utilisateur

        afficher_solde, retrait : méthodes sur la struct Compte