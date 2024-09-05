use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::hashing::hash_password;

pub fn start_brute_force(
    charset: &str, 
    max_length: usize, 
    target_password_hash: &str,
    total_attempts: Arc<Mutex<u64>>, 
    attempts_per_second: Arc<Mutex<u64>>, 
    is_running: Arc<Mutex<bool>>, 
    start: Instant
) {
    let charset_chars: Vec<char> = charset.chars().collect();
    let is_found = Arc::new(Mutex::new(false));

    let mut handles = vec![];

    for length in 1..=max_length {
        let charset_clone = charset_chars.clone();
        let target_hash_clone = target_password_hash.to_string();
        let total_attempts_clone = Arc::clone(&total_attempts);
        let is_found_clone = Arc::clone(&is_found);
        let attempts_per_second_clone = Arc::clone(&attempts_per_second);
        let is_running_clone = Arc::clone(&is_running);

        let handle = thread::spawn(move || {
            for combination in charset_clone.iter().combinations_with_replacement(length) {
                if *is_found_clone.lock().unwrap() {
                    break;
                }
                let attempt = combination.iter().cloned().collect::<String>();
                let attempt_hash = hash_password(&attempt);

                {
                    let mut total = total_attempts_clone.lock().unwrap();
                    *total += 1;
                    let mut attempts = attempts_per_second_clone.lock().unwrap();
                    *attempts += 1;
                }

                if attempt_hash == target_hash_clone {
                    *is_found_clone.lock().unwrap() = true;
                    *is_running_clone.lock().unwrap() = false;
                    let duration = start.elapsed();
                    let total = *total_attempts_clone.lock().unwrap();
                    println!("\rMot de passe trouvé: {} en {:?} secondes, avec {} tentatives", attempt, duration, total);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
