# HashBreaker

HashBreaker est un outil de brute-force de mot de passe écrit en Rust, conçu pour être rapide, flexible et pédagogique. Il permet de tester des mots de passe hachés à l'aide de dictionnaires ou de la génération automatique de combinaisons, en exploitant pleinement la puissance de votre processeur grâce au multi-threading.

## Fonctionnalités principales

- **Support multi-thread** : Utilise tous les cœurs de votre CPU pour accélérer la recherche.
- **Mode streaming** : Peut traiter d'énormes dictionnaires sans les charger entièrement en mémoire.
- **Personnalisation du charset** : Choisissez les types de caractères à utiliser (chiffres, minuscules, majuscules, symboles).
- **Support de plusieurs algorithmes** : MD5, SHA-1, SHA-256, SHA-512, bcrypt, argon2, base64.
- **Télémétrie en temps réel** : Affiche le nombre de tentatives par seconde et la progression.
- **Utilisation de dictionnaires texte ou ZIP** : Prise en charge des fichiers compressés ou non.
- **Détection automatique des dictionnaires** : Le programme détecte automatiquement les dictionnaires présents localement et vous propose de choisir. Si aucun n'est trouvé, il peut télécharger un dictionnaire populaire sur Internet.

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
Algorithme détecté : md5
Voulez-vous utiliser un dictionnaire ? (o/n)
> o
Voulez-vous utiliser un dictionnaire compressé en .zip intégré ? (o/n)
> n
Voulez-vous utiliser un fichier ZIP personnalisé ? (o/n)
> n
Votre dictionnaire est-il trop volumineux pour être chargé en mémoire ? Utiliser le mode streaming ? (o/n)
> n
Votre machine possède 8 cœurs logiques.
Voulez-vous utiliser tous les cœurs disponibles ? (o/n)
> o
Voulez-vous personnaliser le jeu de caractères utilisé pour le brute-force ? (o/n)
> o
Inclure les chiffres ? (o/n)
> o
Inclure les minuscules ? (o/n)
> o
Inclure les majuscules ? (o/n)
> n
Inclure les symboles spéciaux ? (o/n)
> n
Recherche en cours - Tentatives: ... | Tentatives par seconde: ...
```

## Astuces

- Pour utiliser un dictionnaire personnalisé, montez-le dans le conteneur Docker :

  ```bash
  docker run -it --rm -v /chemin/vers/votre/dictionnaire:/app/assets hash-breaker
  ```

- Le mode streaming est recommandé pour les très gros fichiers texte (>1Go).
- Vous pouvez interrompre la recherche à tout moment avec `Ctrl+C`.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus d'informations.
