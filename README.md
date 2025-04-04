# HashBreaker

HashBreaker est une implémentation simple d'un outil de brute-force de mot de passe en Rust. Il utilise des combinaisons de caractères ou des dictionnaires pour essayer de deviner un mot de passe haché jusqu'à ce que le bon mot de passe soit trouvé ou que toutes les combinaisons possibles soient épuisées.

## Fonctionnalités

- Prise en charge de plusieurs algorithmes de hachage (MD5, SHA-1, SHA-256, SHA-512, bcrypt, argon2, base64).
- Saisie du mot de passe haché et choix de l'algorithme via le terminal.
- Utilisation d'un dictionnaire de mots de passe (fichier texte ou fichier compressé en `.zip`).
- Affichage du nombre de tentatives par seconde et des progrès de la recherche brute-force.
- Option pour utiliser un fichier ZIP intégré ou personnalisé comme dictionnaire.

## Avertissements

Ce logiciel est fourni à des fins éducatives et de recherche. L'utilisation de cet outil pour attaquer des cibles sans consentement mutuel préalable est illégale. L'utilisateur final est responsable de respecter toutes les lois locales lors de l'utilisation de cet outil.

## Prérequis

- Docker installé sur votre machine. [Instructions d'installation de Docker](https://docs.docker.com/get-docker/).
- (Facultatif) Rust installé pour exécuter le projet localement. Vous pouvez installer Rust via [rustup](https://rustup.rs/).

## Installation avec Docker

Clonez ce dépôt sur votre machine locale en utilisant :

```bash
git clone https://github.com/leoteissier/hash-breaker.git
cd hash-breaker
```

### Construction de l'image Docker

Utilisez la commande suivante pour construire l'image Docker :

```bash
docker build -t hash-breaker .
```

### Exécution du projet avec Docker

Une fois l'image construite, exécutez le projet en mode interactif :

```bash
docker run -it --rm hash-breaker
```

Suivez les instructions à l'écran pour fournir le mot de passe haché, choisir l'algorithme de hachage, et décider d'utiliser ou non un dictionnaire.

### Utilisation d'un dictionnaire

Si vous souhaitez utiliser un dictionnaire de mots de passe personnalisé, vous pouvez monter un fichier texte ou un fichier `.zip` contenant le dictionnaire dans le conteneur Docker :

```bash
docker run -it --rm -v /chemin/vers/votre/dictionnaire:/app/assets hash-breaker
```

## Exécution locale sans Docker (optionnel)

### Compilation du projet

Si vous avez Rust installé sur votre machine, vous pouvez également exécuter le projet localement. Clonez le dépôt et compilez le projet :

```bash
cargo build --release
```

### Exécution du projet

Exécutez le programme compilé avec la commande suivante :

```bash
./target/release/hash_breaker
```

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus d'informations.
