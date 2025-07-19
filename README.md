# HashBreaker

HashBreaker est un outil de brute-force de mot de passe écrit en Rust, conçu pour être rapide, flexible et pédagogique. Il permet de tester des mots de passe hachés à l'aide de dictionnaires ou de la génération automatique de combinaisons, en exploitant pleinement la puissance de votre processeur grâce au multi-threading.

## Fonctionnalités principales

- **Support multi-thread** : Utilise tous les cœurs de votre CPU pour accélérer la recherche.
- **Détection automatique des dictionnaires** : Le programme détecte automatiquement les dictionnaires présents dans `./` et `./assets/` et vous propose de choisir.
- **Téléchargement automatique** : Si aucun dictionnaire n'est trouvé, possibilité de télécharger automatiquement un dictionnaire populaire (rockyou.txt).
- **Mode streaming** : Peut traiter d'énormes dictionnaires sans les charger entièrement en mémoire.
- **Personnalisation du charset** : Choisissez les types de caractères à utiliser (chiffres, minuscules, majuscules, symboles) - seulement si pas de dictionnaire.
- **Support de plusieurs algorithmes** : MD5, SHA-1, SHA-256, SHA-512, bcrypt, argon2, base64.
- **Télémétrie en temps réel** : Affiche le nombre de tentatives par seconde et la progression.
- **Utilisation de dictionnaires texte ou ZIP** : Prise en charge des fichiers compressés ou non.
- **Interface utilisateur intuitive** : Réponses par défaut clairement indiquées avec `[O]` et `[N]`.

## Exemple d'utilisation rapide

### 1. Cloner le dépôt

```bash
git clone https://github.com/leoteissier/hash-breaker.git
cd hash-breaker
```

### 2. Compiler et lancer (si Rust installé)

```bash
cargo build --release
./target/release/hash_breaker
```

### 3. Ou utiliser Docker

```bash
docker build -t hash-breaker .
docker run -it --rm hash-breaker
```

### 4. Exemple de session interactive

```
Veuillez entrer le mot de passe haché à brute-forcer :
> 5f4dcc3b5aa765d61d8327deb882cf99

Dictionnaires détectés :
  [1] ./assets/passwords.zip
  [2] ./rockyou.txt

Veuillez choisir un dictionnaire (numéro) ou appuyez sur Entrée pour ne pas en utiliser :
> 1

Algorithme détecté : md5

Votre dictionnaire est-il trop volumineux pour être chargé en mémoire ? Utiliser le mode streaming ? (o/n) [N]
>

Votre machine possède 8 cœurs logiques.
Voulez-vous utiliser tous les cœurs disponibles ? (o/n) [O]
>

Recherche en cours - Tentatives: 1234567 | Tentatives par seconde: 45678
```

### 5. Exemple sans dictionnaire (brute-force)

```
Veuillez entrer le mot de passe haché à brute-forcer :
> 5f4dcc3b5aa765d61d8327deb882cf99

Aucun dictionnaire trouvé dans ./ ou ./assets.
Voulez-vous télécharger un dictionnaire populaire (rockyou.txt) ? (o/n)
> n

Algorithme détecté : md5

Voulez-vous personnaliser le jeu de caractères utilisé pour le brute-force ? (o/n) [N]
> o

Inclure les chiffres ? (o/n) [O]
>

Inclure les minuscules ? (o/n) [O]
>

Inclure les majuscules ? (o/n) [O]
>

Inclure les symboles spéciaux ? (o/n) [N]
>

Votre machine possède 8 cœurs logiques.
Voulez-vous utiliser tous les cœurs disponibles ? (o/n) [O]
>

Recherche en cours - Tentatives: 1234567 | Tentatives par seconde: 45678
```

## Réponses par défaut

Le programme utilise des réponses par défaut intuitives :

- `[O]` = Oui par défaut (appuyez sur Entrée pour accepter)
- `[N]` = Non par défaut (appuyez sur Entrée pour accepter)

## Astuces

- **Dictionnaires** : Placez vos dictionnaires dans `./` ou `./assets/` pour qu'ils soient détectés automatiquement.
- **Mode streaming** : Recommandé pour les très gros fichiers texte (>1Go).
- **Interruption** : Vous pouvez interrompre la recherche à tout moment avec `Ctrl+C`.
- **Performance** : Utilisez tous les cœurs disponibles pour de meilleures performances.

## Structure du projet

```
hash-breaker/
├── src/
│   ├── main.rs          # Point d'entrée et interface utilisateur
│   ├── brute_force.rs   # Logique de brute-force multi-thread
│   ├── hashing.rs       # Algorithmes de hachage
│   ├── telemetry.rs     # Affichage des statistiques
│   └── utils.rs         # Utilitaires (gestion des dictionnaires)
├── tests/               # Tests unitaires
├── scripts/             # Scripts de vérification
│   ├── check.sh         # Vérification complète
│   └── quick-check.sh   # Vérification rapide
├── assets/              # Dictionnaires par défaut
├── Makefile             # Commandes de développement
└── Cargo.toml           # Configuration du projet
```

## 🛠️ Développement

### Prérequis

- Rust 1.75.0 ou plus récent
- Git

### Vérification rapide avant commit

```bash
# Vérification rapide (recommandé avant chaque commit)
make quick-check
# ou
./scripts/quick-check.sh
```

### Vérification complète

```bash
# Vérification complète (tests, formatage, clippy, documentation, etc.)
make check
# ou
./scripts/check.sh
```

### Commandes utiles

```bash
make help          # Afficher toutes les commandes disponibles
make test          # Exécuter les tests
make format        # Formater le code
make clippy        # Vérifier avec clippy
make build         # Compiler en mode release
make clean         # Nettoyer les fichiers de build
make doc           # Générer la documentation
make install-tools # Installer les outils de développement
```

## CI/CD et Tests

Ce projet utilise GitHub Actions pour automatiser les tests et les déploiements :

### 🚀 **Tests Automatiques**

- **Tests unitaires** : Vérification de tous les algorithmes de hachage
- **Tests d'intégration** : Validation du brute-force et de la détection d'algorithmes
- **Formatage** : Vérification du style de code avec `rustfmt`
- **Linting** : Analyse statique avec `clippy`
- **Audit de sécurité** : Vérification des vulnérabilités avec `cargo audit`

### 📦 **Build Multi-Plateforme**

- **Linux** (x86_64)
- **macOS** (x86_64)
- **Windows** (x86_64)

### 🎯 **Releases Automatiques**

- Création automatique de releases lors du push de tags `v*`
- Binaires pré-compilés pour toutes les plateformes
- Notes de release générées automatiquement

### 📊 **Benchmarks**

- Tests de performance hebdomadaires
- Monitoring des performances du programme

[![CI/CD Pipeline](https://github.com/leoteissier/hash-breaker/workflows/CI/badge.svg)](https://github.com/leoteissier/hash-breaker/actions)

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus d'informations.
