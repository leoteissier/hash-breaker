mod brute_force;
mod hashing;
mod telemetry;
mod utils;

use utils::load_zipped_dictionary;
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use num_cpus;
use std::fs;

fn detect_dictionaries() -> Vec<String> {
    let mut dicts = Vec::new();
    for dir in &[".", "./assets"] {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "txt" || ext == "zip" {
                        dicts.push(path.display().to_string());
                    }
                }
            }
        }
    }
    dicts
}

fn download_rockyou() -> Option<String> {
    let url = "https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt";
    let dest = "rockyou.txt";
    println!("\x1b[33mTéléchargement du dictionnaire rockyou.txt...\x1b[0m");
    match reqwest::blocking::get(url) {
        Ok(resp) => {
            let mut out = std::fs::File::create(dest).ok()?;
            let content = resp.bytes().ok()?;
            std::io::copy(&mut content.as_ref(), &mut out).ok()?;
            println!("\x1b[32mDictionnaire téléchargé avec succès !\x1b[0m");
            Some(dest.to_string())
        },
        Err(_) => {
            println!("\x1b[31mÉchec du téléchargement du dictionnaire.\x1b[0m");
            None
        }
    }
}

fn main() {
    // Demande à l'utilisateur de fournir le mot de passe haché à brute-forcer
    println!("Veuillez entrer le mot de passe haché à brute-forcer :");
    let mut target_password_hash = String::new();
    stdin().read_line(&mut target_password_hash).unwrap();
    target_password_hash = target_password_hash.trim().to_string();

    // Détection automatique des dictionnaires
    let dictionaries = detect_dictionaries();
    let dictionary_path = if dictionaries.is_empty() {
        println!("\x1b[31mAucun dictionnaire trouvé dans ./ ou ./assets.\x1b[0m");
        println!("Voulez-vous télécharger un dictionnaire populaire (rockyou.txt) ? (o/n)");
        let mut dl_choice = String::new();
        stdin().read_line(&mut dl_choice).unwrap();
        if dl_choice.trim().to_lowercase() == "o" {
            download_rockyou()
        } else {
            None
        }
    } else {
        println!("\x1b[34mDictionnaires détectés :\x1b[0m");
        for (i, dict) in dictionaries.iter().enumerate() {
            println!("  [{}] {}", i + 1, dict);
        }
        println!("Veuillez choisir un dictionnaire (numéro) ou appuyez sur Entrée pour ne pas en utiliser :");
        let mut dict_choice = String::new();
        stdin().read_line(&mut dict_choice).unwrap();
        if let Ok(idx) = dict_choice.trim().parse::<usize>() {
            if idx > 0 && idx <= dictionaries.len() {
                Some(dictionaries[idx - 1].clone())
            } else {
                None
            }
        } else {
            None
        }
    };

    let dictionary: Option<Vec<String>> = if let Some(path) = dictionary_path {
        if path.ends_with(".zip") {
            Some(load_zipped_dictionary(&path))
        } else {
            // Mode streaming sera proposé plus loin
            Some(fs::read_to_string(&path).unwrap().lines().map(|l| l.to_string()).collect())
        }
    } else {
        None
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

    // Demande à l'utilisateur de personnaliser le charset SEULEMENT si pas de dictionnaire
    let charset = if dictionary.is_none() {
        println!("Voulez-vous personnaliser le jeu de caractères utilisé pour le brute-force ? (o/n) [N]");
        let mut custom_charset = String::new();
        stdin().read_line(&mut custom_charset).unwrap();
        let custom_charset = custom_charset.trim().to_lowercase();
        if custom_charset == "o" {
            println!("Inclure les chiffres ? (o/n) [O]");
            let mut chiffres = String::new();
            stdin().read_line(&mut chiffres).unwrap();
            let chiffres = chiffres.trim().to_lowercase();
            let chiffres = chiffres.is_empty() || chiffres == "o";
            
            println!("Inclure les minuscules ? (o/n) [O]");
            let mut minuscules = String::new();
            stdin().read_line(&mut minuscules).unwrap();
            let minuscules = minuscules.trim().to_lowercase();
            let minuscules = minuscules.is_empty() || minuscules == "o";
            
            println!("Inclure les majuscules ? (o/n) [O]");
            let mut majuscules = String::new();
            stdin().read_line(&mut majuscules).unwrap();
            let majuscules = majuscules.trim().to_lowercase();
            let majuscules = majuscules.is_empty() || majuscules == "o";
            
            println!("Inclure les symboles spéciaux ? (o/n) [N]");
            let mut symboles = String::new();
            stdin().read_line(&mut symboles).unwrap();
            let symboles = symboles.trim().to_lowercase();
            let symboles = symboles == "o";
            
            let mut cs = String::new();
            if chiffres { cs.push_str("0123456789"); }
            if minuscules { cs.push_str("abcdefghijklmnopqrstuvwxyz"); }
            if majuscules { cs.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
            if symboles { cs.push_str("!@#$%^&*()_+-=[]{}|;:',.<>/?"); }
            if cs.is_empty() {
                println!("Aucun jeu de caractères sélectionné, utilisation du jeu complet par défaut.");
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?".to_string()
            } else {
                cs
            }
        } else {
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?".to_string()
        }
    } else {
        // Si on a un dictionnaire, on utilise un charset vide car on ne fait que du dictionnaire
        String::new()
    };

    // Demande à l'utilisateur s'il souhaite utiliser un mode gros dictionnaire (streaming)
    let mut use_streaming = String::new();
    println!("Votre dictionnaire est-il trop volumineux pour être chargé en mémoire ? Utiliser le mode streaming ? (o/n) [N]");
    stdin().read_line(&mut use_streaming).unwrap();
    let use_streaming = use_streaming.trim().to_lowercase() == "o";
    let mut streaming_path = String::new();
    if use_streaming {
        println!("Veuillez entrer le chemin complet du fichier dictionnaire texte :");
        stdin().read_line(&mut streaming_path).unwrap();
        streaming_path = streaming_path.trim().to_string();
    }

    let total_cores = num_cpus::get();
    println!("Votre machine possède {} cœurs logiques.", total_cores);
    println!("Voulez-vous utiliser tous les cœurs disponibles ? (o/n) [O]");
    let mut use_all_cores = String::new();
    stdin().read_line(&mut use_all_cores).unwrap();
    let use_all_cores = use_all_cores.trim().to_lowercase();
    let num_threads = if use_all_cores.is_empty() || use_all_cores == "o" {
        total_cores
    } else {
        println!("Combien de cœurs souhaitez-vous utiliser ? (1-{})", total_cores);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<usize>().unwrap_or(1);
        if n > 0 && n <= total_cores { n } else { 1 }
    };

    let is_running = Arc::new(Mutex::new(true));
    let attempts_per_second = Arc::new(Mutex::new(0));
    let total_attempts = Arc::new(Mutex::new(0));

    // Télémétrie simple qui fonctionne
    let telemetry_handle = telemetry::start_telemetry_thread(
        Arc::clone(&is_running),
        Arc::clone(&total_attempts),
        Arc::clone(&attempts_per_second),
    );
    // Le spinner peut rester inchangé
    let spinner_handle = telemetry::start_spinner_thread(
        Arc::clone(&is_running),
        Arc::clone(&total_attempts),
    );

    let start = Instant::now();

    // Execute brute-force with the given dictionary and algorithm
    brute_force::start_brute_force(
        &charset, 
        &target_password_hash, 
        &algorithm,
        Arc::clone(&total_attempts), 
        Arc::clone(&attempts_per_second), 
        Arc::clone(&is_running),
        dictionary,
        use_streaming,
        streaming_path,
        num_threads,
    );

    // Stop telemetry and spinner once the brute-force process ends
    let duration = start.elapsed();
    *is_running.lock().unwrap() = false;
    telemetry_handle.join().unwrap();
    spinner_handle.join().unwrap();
    
    // Display results
    let total = *total_attempts.lock().unwrap();
    let _per_sec = *attempts_per_second.lock().unwrap();
    println!("\x1b[32m\nRecherche complétée en {:?} secondes avec {} tentatives\x1b[0m", duration, total);
    println!("\x1b[1;33mSi le mot de passe a été trouvé, il est affiché ci-dessus.\x1b[0m");
}
