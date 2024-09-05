mod brute_force;
mod hashing;
mod telemetry;

use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    println!("Veuillez entrer le mot de passe haché à brute-forcer (SHA-256) :");
    let mut target_password_hash = String::new();
    stdin().read_line(&mut target_password_hash).unwrap();
    target_password_hash = target_password_hash.trim().to_string();

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?";
    let max_length = 8; // Limite de la longueur du mot de passe

    let is_running = Arc::new(Mutex::new(true));
    let attempts_per_second = Arc::new(Mutex::new(0));
    let total_attempts = Arc::new(Mutex::new(0));

    let telemetry_handle = telemetry::start_telemetry_thread(
        Arc::clone(&attempts_per_second), 
        Arc::clone(&is_running)
    );

    let handle = telemetry::start_spinner_thread(
        Arc::clone(&is_running),
        Arc::clone(&total_attempts)
    );

    let start = Instant::now();

    brute_force::start_brute_force(
        charset, 
        max_length, 
        &target_password_hash, 
        Arc::clone(&total_attempts), 
        Arc::clone(&attempts_per_second), 
        Arc::clone(&is_running),
        start
    );

    let duration = start.elapsed();
    *is_running.lock().unwrap() = false;
    telemetry_handle.join().unwrap();
    handle.join().unwrap();
    let total = *total_attempts.lock().unwrap();

    println!("\rMot de passe non trouvé, recherche complétée en {:?} secondes avec {} tentatives", duration, total);
}
