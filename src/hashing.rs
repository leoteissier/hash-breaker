use md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};
use base64::{engine::general_purpose, Engine as _};
use bcrypt::hash as bcrypt_hash;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

/// Hash une chaîne de caractères en fonction de l'algorithme spécifié
pub fn hash_password(password: &str, algorithm: &str) -> String {
    match algorithm.to_lowercase().as_str() {
        "md5" => {
            let digest = md5::compute(password);
            format!("{:x}", digest)
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(password.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(password.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "bcrypt" => {
            let hashed = bcrypt_hash(password, 4).unwrap();
            hashed
        }
        "argon2" => {
            // Générer un sel sécurisé
            let salt = SaltString::generate(&mut OsRng);

            // Utiliser Argon2 pour hacher le mot de passe
            let argon2 = Argon2::default();
            let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
            hash.to_string()
        }
        "base64" => {
            let encoded = general_purpose::STANDARD.encode(password);
            encoded
        }
        _ => {
            panic!("Algorithme non supporté : {}", algorithm);
        }
    }
}

/// Détecte l'algorithme en fonction de la longueur du hash ou s'il est en Base64
pub fn detect_algorithm(hash: &str) -> Result<String, &'static str> {
    // Ajout de la vérification de la longueur minimale pour les hachages hexadécimaux
    if is_hex(hash) {
        // Vérifier si la longueur est suffisante pour être un hachage hexadécimal
        return match hash.len() {
            32 => Ok(String::from("md5")),     // 32 caractères -> MD5
            40 => Ok(String::from("sha1")),    // 40 caractères -> SHA-1
            64 => Ok(String::from("sha256")),  // 64 caractères -> SHA-256
            128 => Ok(String::from("sha512")), // 128 caractères -> SHA-512
            _ if hash.len() < 32 => Err("La chaîne est trop courte pour être un hachage valide"), // Trop court
            _ => Err("Longueur de hachage hexadécimal non reconnue"),  // Longueur non reconnue
        };
    }

    // Vérifie les préfixes spécifiques aux hachages bcrypt et argon2
    if hash.starts_with("$2b$") || hash.starts_with("$2a$") {
        return Ok(String::from("bcrypt"));
    }
    if hash.starts_with("$argon2") {
        return Ok(String::from("argon2"));
    }

    // Vérifie si la chaîne est encodée en Base64
    if general_purpose::STANDARD.decode(hash).is_ok() {
        return Ok(String::from("base64"));
    }

    Err("Longueur de hachage ou format non reconnu")
}

/// Vérifie si une chaîne est un hexadécimal valide
fn is_hex(hash: &str) -> bool {
    hash.chars().all(|c| c.is_digit(16))
}
