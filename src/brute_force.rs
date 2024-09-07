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
        if !dict.is_empty() {
            for (_i, word) in dict.iter().enumerate() {
                // Convert word from bytes if necessary
                let word_str = String::from_utf8_lossy(word.as_bytes()).to_string();
                let attempt_hash = hash_password(&word_str, algorithm);
    
                // Mises à jour atomiques des compteurs
                {
                    let mut total = total_attempts.lock().unwrap();
                    *total += 1;
    
                    let mut attempts_per_sec = attempts_per_second.lock().unwrap();
                    *attempts_per_sec += 1;
                }
    
                // Vérification si le mot de passe est trouvé
                if attempt_hash == target_password_hash {
                    println!("\nMot de passe trouvé : {}", word_str);
                    *is_running.lock().unwrap() = false;
                    return;
                }
            }
            *is_running.lock().unwrap() = false;
            return;
        }
    }

    // If no dictionary, use brute-force character generation
    for length in 1.. {
        for attempt in generate_combinations_iter(charset, length) {

            let attempt_hash = hash_password(&attempt, algorithm);

            {
                let mut total = total_attempts.lock().unwrap();
                *total += 1;

                let mut attempts_per_sec = attempts_per_second.lock().unwrap();
                *attempts_per_sec += 1;
            }

            if attempt_hash == target_password_hash {
                println!("\nMot de passe trouvé : {}", attempt);
                *is_running.lock().unwrap() = false;
                return;
            }
        }
    }

    *is_running.lock().unwrap() = false;
}


/// Générer les combinaisons de caractères à la volée
pub fn generate_combinations_iter(charset: &str, length: usize) -> Box<dyn Iterator<Item = String>> {
    let charset_vec: Vec<char> = charset.chars().collect();
    Box::new((0..charset_vec.len().pow(length as u32)).map(move |i| {
        let mut result = String::with_capacity(length);
        let mut num = i;
        for _ in 0..length {
            result.push(charset_vec[num % charset_vec.len()]);
            num /= charset_vec.len();
        }
        result
    }))
}
