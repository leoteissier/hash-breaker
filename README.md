# Rust Unlocker

Ce projet est une implémentation simple d'un outil de brute-force de mot de passe en Rust. Il utilise des combinaisons de caractères pour essayer de deviner un mot de passe fourni par l'utilisateur jusqu'à ce que le bon mot de passe soit trouvé ou que toutes les combinaisons possibles soient épuisées.

## Fonctionnalités

- Saisie du mot de passe cible via le terminal.
- Affichage du nombre de tentatives par seconde.
- Affichage animé indiquant la progression de la recherche.

## Avertissements

Ce logiciel est fourni à des fins éducatives et de recherche. L'utilisation de cet outil pour attaquer des cibles sans consentement mutuel préalable est illégale. L'utilisateur final est responsable de respecter toutes les lois locales lors de l'utilisation de cet outil.

## Prérequis

Pour exécuter ce projet, vous aurez besoin de Rust installé sur votre machine. Vous pouvez installer Rust via [rustup](https://rustup.rs/).

## Installation

Clonez ce dépôt sur votre machine locale en utilisant :

```bash
git clone https://github.com/leoteissier/RustUnlocker.git
cd RustUnlocker
```

Compilez le projet en utilisant Cargo :

```bash
cargo build --release
```

## Utilisation

Exécutez le programme en utilisant :

```bash
./target/release/rust_unlocker
```

Suivez les instructions à l'écran pour fournir le mot de passe cible et démarrer la recherche.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus d'informations.
