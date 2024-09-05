use std::sync::{Arc, Mutex};
use crate::hashing::hash_password;

pub fn start_brute_force(
    charset: &str,
    target_password_hash: &str,
    algorithm: &str,
    total_attempts: Arc<Mutex<u64>>,
    attempts_per_second: Arc<Mutex<u64>>,
    is_running: Arc<Mutex<bool>>,
    dictionary: Option<Vec<String>>,  // Option pour le dictionnaire
) {
    if let Some(dict) = dictionary {
        // Si un dictionnaire est fourni, ne génère pas de combinaisons, mais utilise le dictionnaire
        for word in dict {
            let attempt_hash = hash_password(&word, algorithm);
            *total_attempts.lock().unwrap() += 1;
            *attempts_per_second.lock().unwrap() += 1;

            if attempt_hash == target_password_hash {
                println!("\nMot de passe trouvé : {}", word);
                *is_running.lock().unwrap() = false;
                return;
            }
        }
        // Si le mot de passe n'est pas trouvé dans le dictionnaire, on arrête la recherche
        *is_running.lock().unwrap() = false;
        return;
    }

    // Si aucun dictionnaire n'est fourni, générer toutes les combinaisons possibles
    for length in 1.. {
        let combinations = generate_combinations(charset, length);
        for attempt in combinations {
            let attempt_hash = hash_password(&attempt, algorithm);
            *total_attempts.lock().unwrap() += 1;
            *attempts_per_second.lock().unwrap() += 1;

            if attempt_hash == target_password_hash {
                println!("\nMot de passe trouvé : {}", attempt);
                *is_running.lock().unwrap() = false;
                return;
            }
        }
    }
    *is_running.lock().unwrap() = false;
}


// Fonction récursive pour générer toutes les combinaisons possibles de caractères
pub fn generate_combinations(charset: &str, length: usize) -> Vec<String> {
    let mut combinations = Vec::new();
    generate_combinations_recursive(charset, length, String::new(), &mut combinations);
    combinations
}

// Fonction récursive qui génère les combinaisons
fn generate_combinations_recursive(charset: &str, length: usize, current: String, combinations: &mut Vec<String>) {
    if length == 0 {
        combinations.push(current);
    } else {
        for c in charset.chars() {
            let mut next = current.clone();
            next.push(c);
            generate_combinations_recursive(charset, length - 1, next, combinations);
        }
    }
}

