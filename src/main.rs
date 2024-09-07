mod brute_force;
mod hashing;
mod telemetry;
mod file_utils;

use file_utils::{load_zipped_dictionary, load_zipped_dictionary_from_embedded};
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    // Demande à l'utilisateur de fournir le mot de passe haché à brute-forcer
    println!("Veuillez entrer le mot de passe haché à brute-forcer :");
    let mut target_password_hash = String::new();
    stdin().read_line(&mut target_password_hash).unwrap();
    target_password_hash = target_password_hash.trim().to_string();

    // Demande à l'utilisateur s'il souhaite utiliser un dictionnaire
    let dictionary: Option<Vec<String>> = loop {
        println!("Voulez-vous utiliser un dictionnaire pour brute-forcer le mot de passe ? (o/n)");
        let mut use_dictionary = String::new();
        stdin().read_line(&mut use_dictionary).unwrap();
        let use_dictionary = use_dictionary.trim().to_string().to_lowercase();

        if use_dictionary == "o" {
            // Demande si l'utilisateur veut utiliser un fichier ZIP ou un fichier texte
            println!("Voulez-vous utiliser un dictionnaire compressé en .zip intégré ? (o/n)");
            let mut choice = String::new();
            stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().to_string().to_lowercase();

            if choice == "o" {
                // Utilisation du dictionnaire compressé intégré dans le binaire
                break Some(load_zipped_dictionary_from_embedded());
            } else {
                // Demander si l'utilisateur veut utiliser un fichier ZIP ou un fichier texte
                println!("Voulez-vous utiliser un fichier ZIP personnalisé ? (o/n)");
                let mut zip_choice = String::new();
                stdin().read_line(&mut zip_choice).unwrap();
                let zip_choice = zip_choice.trim().to_string().to_lowercase();

                if zip_choice == "o" {
                    // Utilisation d'un fichier ZIP personnalisé
                    println!("Veuillez entrer le chemin complet du fichier ZIP (y compris le fichier) :");
                    let mut zip_path = String::new();
                    std::io::stdin().read_line(&mut zip_path).unwrap();
                    zip_path = zip_path.trim().to_string();

                    break Some(load_zipped_dictionary(&zip_path));
                } else {
                    // Retourner à la sélection du dictionnaire si "non" pour le ZIP
                    continue;
                }
            }
        } else if use_dictionary == "n" {
            // L'utilisateur ne souhaite pas utiliser de dictionnaire
            break None;
        } else {
            println!("Réponse non valide. Veuillez entrer 'o' pour oui ou 'n' pour non.");
        }
    };

    // Tentative de détection automatique de l'algorithme de hachage
    let algorithm = match hashing::detect_algorithm(&target_password_hash) {
        Ok(algo) => {
            println!("Algorithme détecté : {}", algo);
            algo
        }
        Err(err) => {
            println!("Erreur de détection automatique : {}. Veuillez spécifier l'algorithme (md5, sha1, sha256, sha512, bcrypt, argon2, base64) :", err);
            let mut forced_algorithm = String::new();
            stdin().read_line(&mut forced_algorithm).unwrap();
            forced_algorithm.trim().to_string()
        }
    };

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?";
    let is_running = Arc::new(Mutex::new(true));
    let attempts_per_second = Arc::new(Mutex::new(0));
    let total_attempts = Arc::new(Mutex::new(0));

    // Start telemetry thread to monitor attempts per second
    let telemetry_handle = telemetry::start_telemetry_thread(
        Arc::clone(&is_running),
        Arc::clone(&total_attempts),
        Arc::clone(&attempts_per_second),
    );

    let spinner_handle = telemetry::start_spinner_thread(
        Arc::clone(&is_running),
        Arc::clone(&total_attempts),
    );

    let start = Instant::now();

    // Execute brute-force with the given dictionary and algorithm
    brute_force::start_brute_force(
        charset, 
        &target_password_hash, 
        &algorithm,
        Arc::clone(&total_attempts), 
        Arc::clone(&attempts_per_second), 
        Arc::clone(&is_running),
        dictionary,
    );

    // Stop telemetry and spinner once the brute-force process ends
    let duration = start.elapsed();
    *is_running.lock().unwrap() = false;
    telemetry_handle.join().unwrap();
    spinner_handle.join().unwrap();
    
    // Display results
    let total = *total_attempts.lock().unwrap();
    println!("\rRecherche complétée en {:?} secondes avec {} tentatives", duration, total);
}
