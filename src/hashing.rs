use sha2::{Sha256, Digest}; // Utilisation de SHA-256

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize()) // Hachage en hexadécimal
}
