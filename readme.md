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


# TP3 - Gestion de fichiers en Rust 🦀

Ce projet est une application en ligne de commande écrite en **Rust**, permettant de :

- Créer un fichier texte
- Lire le contenu du fichier
- Écrire du texte dans le fichier
- Supprimer le fichier
- Afficher la date actuelle (UTC) avec [`chrono`]

## 🧱 Fonctionnalités

- ✅ Gestion de fichiers avec `std::fs`
- ✅ Support UTF-8 (écriture et lecture des accents)
- ✅ Affichage de la date/heure en UTC
- ✅ Menu interactif en boucle (`loop` + `match`)
- ✅ Utilisation d’une structure `Fichier` avec `impl`

# TP7 - Gestion de fichiers en Rust 🦀


# Serveur DNS Simple en Rust

Un serveur DNS minimaliste implémenté en Rust utilisant Tokio pour la programmation asynchrone.

## Description

Ce programme implémente un serveur DNS basique qui peut répondre aux requêtes DNS de type A (résolution nom de domaine vers adresse IPv4). Il écoute sur le port 5354 en local et maintient une table de correspondance statique entre noms de domaines et adresses IP.

## Fonctionnalités

- **Serveur UDP asynchrone** : Utilise Tokio pour gérer les connexions UDP de manière non-bloquante
- **Parsing des requêtes DNS** : Analyse les paquets DNS entrants pour extraire le nom de domaine demandé
- **Table de résolution statique** : Contient des mappings prédéfinis nom de domaine → adresse IP
- **Génération de réponses DNS** : Construit des réponses DNS conformes au protocole
- **Gestion des erreurs** : Ignore les requêtes malformées et continue le service

## Configuration par défaut

Le serveur est configuré avec les enregistrements DNS suivants :

- `example.com` → `192.168.1.1`
- `test.local` → `10.0.0.1`

## Prérequis

- Rust (édition 2021 ou plus récente)
- Tokio runtime

## Installation et compilation

```bash
# Cloner ou télécharger le code source
# Ajouter tokio au Cargo.toml :
[dependencies]
tokio = { version = "1", features = ["full"] }

# Compiler le projet
cargo build --release
```

## Utilisation

### Démarrage du serveur

```bash
cargo run
```

Le serveur démarre et affiche :
```
Serveur DNS écoute sur 127.0.0.1:5354
```

### Test du serveur

Vous pouvez tester le serveur avec des outils comme `dig` ou `nslookup` :

```bash
# Avec dig
dig @127.0.0.1 -p 5354 example.com

# Avec nslookup
nslookup example.com 127.0.0.1:5354
```

## Structure du code

### Fonction principale (`main`)
- Crée et lie le socket UDP sur `127.0.0.1:5354`
- Initialise la table des enregistrements DNS
- Boucle principale de réception et traitement des requêtes

### Fonction `parse_query`
- Extrait l'ID de transaction et le nom de domaine depuis un paquet DNS
- Gère le format des labels DNS (longueur + données)
- Retourne `None` pour les paquets malformés

### Fonction `build_response`
- Construit une réponse DNS complète selon le protocole RFC
- Inclut l'en-tête DNS, la section question et la section réponse
- Utilise des pointeurs pour optimiser la taille du paquet

## Limitations

- **Enregistrements statiques** : La table des domaines est codée en dur
- **Type A uniquement** : Ne supporte que les requêtes IPv4 (type A)
- **Pas de récursion** : Ne fait pas de requêtes vers d'autres serveurs DNS
- **Pas de cache** : Aucune mise en cache des réponses
- **Sécurité limitée** : Pas de validation avancée ou de protection

## Améliorations possibles

- Chargement dynamique des enregistrements depuis un fichier
- Support des types d'enregistrements AAAA (IPv6), MX, CNAME, etc.
- Implémentation de la récursion DNS
- Ajout d'un système de cache
- Interface d'administration pour modifier les enregistrements
- Logging plus détaillé
- Support du protocole TCP pour les grandes réponses

## Port d'écoute

Le serveur utilise le port 5354 (au lieu du port standard 53) pour éviter les conflits avec les serveurs DNS système et permettre l'exécution sans privilèges administrateur.

# TP7 : Transfert de Fichiers TCP

Client/serveur de transfert de fichiers en Rust avec Tokio.

## Description

Système simple de transfert de fichiers via TCP :
- **Serveur** : Reçoit les fichiers et les sauvegarde dans `uploads/`
- **Client** : Envoie un fichier au serveur

## Installation

```bash
# Ajouter au Cargo.toml :
[dependencies]
tokio = { version = "1", features = ["full"] }

cargo build --release
```

## Utilisation

### Serveur
```bash
cargo run --bin server
# Écoute sur 127.0.0.1:4000
```

### Client
```bash
cargo run --bin client <fichier>
# Exemple :
cargo run --bin client document.pdf
```

## Protocole

Format binaire :
1. Opcode (1 byte) : `0x01`
2. Longueur nom (2 bytes)
3. Nom fichier (variable)
4. Taille fichier (8 bytes)
5. Contenu fichier (variable)

## Fonctionnalités

- Transfert asynchrone avec Tokio
- Connexions multiples simultanées
- Création automatique du dossier `uploads/`
- Gestion des erreurs de réseau
- Buffer de 4KB pour l'efficacité

## Limitations

- Pas d'authentification
- Pas de reprise de transfert
- Écrase les fichiers existants
- Protocole propriétaire simple

## TP4 / Projet : Serveur TCP Asynchrone avec Logging de message en Rust 

Ce projet implémente un serveur TCP asynchrone en Rust utilisant Tokio. Le serveur écoute sur l'adresse 127.0.0.1:8080, accepte des connexions entrantes, lit les messages envoyés par les clients, puis les enregistre dans un fichier de log horodaté.
Fonctionnalités principales

    Serveur TCP asynchrone capable de gérer plusieurs clients simultanément grâce à Tokio.

    Logging concurrent des messages reçus depuis chaque client, avec un horodatage précis.

    Les logs sont stockés dans un fichier logs/server.log.

    Création automatique du dossier logs s'il n'existe pas.

    Protection d'accès au fichier log via un mutex asynchrone pour éviter les conflits d'écriture.

## Dépendances

    Tokio pour la programmation asynchrone.

    Chrono pour la gestion des timestamps.

## Utilisation

    Assurez-vous d'avoir Rust et Cargo installés.

    Compilez le projet avec cargo build --release.

    Lancez le serveur avec cargo run --release.

    Connectez des clients TCP à l'adresse 127.0.0.1:8080.

    Les messages reçus seront ajoutés au fichier logs/server.log.

Remarque importante

Ce code a été présenté au professeur lors du premier examen, dans le cadre de la démonstration du projet.