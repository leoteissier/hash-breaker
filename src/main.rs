mod brute_force;
mod hashing;
mod telemetry;
mod file_utils;

use file_utils::{load_zipped_dictionary, load_zipped_dictionary_from_embedded};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    // Demande à l'utilisateur de fournir le mot de passe haché à brute-forcer
    println!("Veuillez entrer le mot de passe haché à brute-forcer :");
    let mut target_password_hash = String::new();
    stdin().read_line(&mut target_password_hash).unwrap();
    target_password_hash = target_password_hash.trim().to_string();

    // Tentative de détection automatique de l'algorithme de hachage
    let algorithm = match hashing::detect_algorithm(&target_password_hash) {
        Ok(algo) => {
            println!("Algorithme détecté : {}", algo);
            algo
        }
        Err(err) => {
            // Si la détection échoue, demander à l'utilisateur de spécifier l'algorithme
            println!("Erreur de détection automatique : {}. Veuillez spécifier l'algorithme (md5, sha1, sha256, sha512, bcrypt, argon2, base64) :", err);
            let mut forced_algorithm = String::new();
            stdin().read_line(&mut forced_algorithm).unwrap();
            forced_algorithm.trim().to_string()
        }
    };

    // Demande à l'utilisateur s'il souhaite utiliser un dictionnaire compressé en .zip
    println!("Voulez-vous utiliser un dictionnaire compressé en .zip intégré ? (o/n)");
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_string();

    let dictionary: Vec<String>;

    if choice.to_lowercase() == "o" {
        // Utilisation du dictionnaire compressé intégré dans le binaire
        dictionary = load_zipped_dictionary_from_embedded();
    } else {
        // Demande à l'utilisateur s'il souhaite utiliser un fichier ZIP personnalisé ou un fichier texte
        println!("Voulez-vous utiliser un fichier ZIP personnalisé ? (o/n)");
        let mut zip_choice = String::new();
        stdin().read_line(&mut zip_choice).unwrap();
        zip_choice = zip_choice.trim().to_string();

        if zip_choice.to_lowercase() == "o" {
            // Utilisation d'un fichier ZIP personnalisé
            println!("Veuillez entrer le chemin du fichier ZIP :");
            let mut zip_path = String::new();
            stdin().read_line(&mut zip_path).unwrap();
            zip_path = zip_path.trim().to_string();

            println!("Veuillez entrer le nom du fichier dans le ZIP :");
            let mut file_in_zip = String::new();
            stdin().read_line(&mut file_in_zip).unwrap();
            file_in_zip = file_in_zip.trim().to_string();

            dictionary = load_zipped_dictionary(&zip_path, &file_in_zip);
        } else {
            // Charger un fichier de dictionnaire personnalisé
            println!("Veuillez entrer le chemin du fichier dictionnaire :");
            let mut dictionary_path = String::new();
            stdin().read_line(&mut dictionary_path).unwrap();
            dictionary_path = dictionary_path.trim().to_string();
            dictionary = load_dictionary(&dictionary_path);
        }
    }

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?";
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

    // Exécution de la brute-force avec le mot de passe et l'algorithme détectés ou spécifiés
    brute_force::start_brute_force(
        charset, 
        &target_password_hash, 
        &algorithm,
        Arc::clone(&total_attempts), 
        Arc::clone(&attempts_per_second), 
        Arc::clone(&is_running),
        Some(dictionary), // Utilisation du dictionnaire choisi
    );

    let duration = start.elapsed();
    *is_running.lock().unwrap() = false;
    telemetry_handle.join().unwrap();
    handle.join().unwrap();
    let total = *total_attempts.lock().unwrap();

    println!("\rMot de passe non trouvé, recherche complétée en {:?} secondes avec {} tentatives", duration, total);
}

// Fonction pour charger un dictionnaire personnalisé depuis un fichier
fn load_dictionary(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Impossible d'ouvrir le fichier dictionnaire");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.expect("Impossible de lire la ligne"))
        .collect()
}
