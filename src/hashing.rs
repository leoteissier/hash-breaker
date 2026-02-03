use argon2::{
    password_hash::PasswordVerifier,
    Argon2,
};
use argon2::password_hash::{PasswordHash, PasswordHasher, SaltString};
use argon2::password_hash::rand_core::OsRng;
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash as bcrypt_hash, verify as bcrypt_verify};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

/// Hash une chaîne de caractères en fonction de l'algorithme spécifié.
/// Pour bcrypt et argon2, génère un nouveau hash (utile pour les tests).
/// Pour le brute-force, utiliser `verify_password` qui compare directement au hash cible.
#[allow(dead_code)]
pub fn hash_password(
    password: &str,
    algorithm: &str,
    salt: &str,
    salt_position: SaltPosition,
) -> String {
    let input = build_salted_input(password, salt, salt_position);
    match algorithm.to_lowercase().as_str() {
        "md5" => {
            let digest = md5::compute(input.as_bytes());
            format!("{digest:x}")
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "bcrypt" => bcrypt_hash(&input, 4).unwrap(),
        "argon2" => {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let hash = argon2.hash_password(input.as_bytes(), &salt).unwrap();
            hash.to_string()
        }
        "base64" => general_purpose::STANDARD.encode(&input),
        _ => {
            panic!("Algorithme non supporté : {algorithm}");
        }
    }
}

/// Vérifie si un candidat correspond au hash cible.
/// Pour bcrypt et argon2, utilise la vérification native (le salt est dans le hash).
/// Pour les autres algorithmes, hash le candidat avec le salt et compare.
pub fn verify_password(
    candidate: &str,
    target_hash: &str,
    algorithm: &str,
    salt: &str,
    salt_position: SaltPosition,
) -> bool {
    let algo = algorithm.to_lowercase();
    match algo.as_str() {
        "bcrypt" => bcrypt_verify(candidate, target_hash).unwrap_or(false),
        "argon2" => {
            let parsed = match PasswordHash::new(target_hash) {
                Ok(p) => p,
                Err(_) => return false,
            };
            Argon2::default()
                .verify_password(candidate.as_bytes(), &parsed)
                .is_ok()
        }
        "md5" | "sha1" | "sha256" | "sha512" | "base64" => {
            let input = build_salted_input(candidate, salt, salt_position);
            match algo.as_str() {
                "md5" => {
                    let digest = md5::compute(input.as_bytes());
                    format!("{digest:x}") == target_hash
                }
                "sha1" => {
                    let mut hasher = Sha1::new();
                    hasher.update(input.as_bytes());
                    format!("{:x}", hasher.finalize()) == target_hash
                }
                "sha256" => {
                    let mut hasher = Sha256::new();
                    hasher.update(input.as_bytes());
                    format!("{:x}", hasher.finalize()) == target_hash
                }
                "sha512" => {
                    let mut hasher = Sha512::new();
                    hasher.update(input.as_bytes());
                    format!("{:x}", hasher.finalize()) == target_hash
                }
                "base64" => general_purpose::STANDARD.encode(&input) == target_hash,
                _ => false,
            }
        }
        _ => false,
    }
}

fn build_salted_input(password: &str, salt: &str, salt_position: SaltPosition) -> String {
    match salt_position {
        SaltPosition::Before => format!("{salt}{password}"),
        SaltPosition::After => format!("{password}{salt}"),
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
            _ => Err("Longueur de hachage hexadécimal non reconnue"), // Longueur non reconnue
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
    hash.chars().all(|c| c.is_ascii_hexdigit())
}

#[derive(Copy, Clone)]
pub enum SaltPosition {
    Before,
    After,
}
