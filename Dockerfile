# Étape 1 : Utiliser une image Rust officielle pour la compilation
FROM rust:1.80-slim-bullseye AS build

# Définir le répertoire de travail à la racine
WORKDIR /app

# Copier les fichiers Cargo.toml et Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances (cela permet de garder les dépendances en cache)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copier le reste du projet
COPY . .

# Compiler l'application en mode release
RUN cargo build --release

# Étape 2 : Créer une image plus légère avec seulement le binaire
FROM debian:buster-slim

# Copier le binaire depuis l'étape de compilation
COPY --from=build /app/target/release/hash_breaker /app/hash_breaker

# Définir le répertoire de travail
WORKDIR /app

# Exécuter le binaire
CMD ["./hash_breaker"]