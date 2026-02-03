# Étape 1 : Utiliser une image Rust officielle pour la compilation
FROM rust:1.82-slim-bullseye AS build

# Installer les dépendances système nécessaires à la compilation de crates (openssl, pkg-config, etc.)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

# Définir le répertoire de travail à la racine
WORKDIR /app

# Copier les fichiers Cargo.toml et Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copier le reste du projet
COPY . .

# Compiler l'application en mode release
RUN cargo build --release

# Étape 2 : Créer une image plus légère avec seulement le binaire
FROM debian:bullseye-slim

# Installer les bibliothèques runtime (libssl pour reqwest/bcrypt, ca-certificates pour HTTPS)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copier le binaire depuis l'étape de compilation
COPY --from=build /app/target/release/hash_breaker /app/hash_breaker

# Définir le répertoire de travail
WORKDIR /app

# ENTRYPOINT permet de passer des arguments (ex: --help) au binaire
ENTRYPOINT ["./hash_breaker"]