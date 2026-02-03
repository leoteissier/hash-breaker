mod brute_force;
mod config;
mod hashing;
mod telemetry;
mod utils;

use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use utils::load_zipped_dictionary;

use clap::Parser;

/// HashBreaker - Outil de brute-force de mot de passe
#[derive(Parser, Debug)]
#[command(name = "hash-breaker")]
#[command(author, version, about = "Outil de brute-force de mot de passe écrit en Rust")]
struct Args {
    /// Hash du mot de passe à cracker (évite la question interactive)
    #[arg(long)]
    hash: Option<String>,

    /// Chemin vers le dictionnaire (évite la sélection interactive)
    #[arg(long, short)]
    dictionary: Option<String>,

    /// Algorithme de hachage (md5, sha1, sha256, sha512, bcrypt, argon2, base64)
    #[arg(long, short)]
    algorithm: Option<String>,

    /// Nombre de threads à utiliser
    #[arg(long, short)]
    threads: Option<usize>,

    /// Mode non-interactif (utilise les valeurs par défaut pour les questions)
    #[arg(long)]
    non_interactive: bool,
}

fn download_rockyou() -> Option<String> {
    let url = "https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt";

    // Créer le dossier dictionaries s'il n'existe pas
    if !std::path::Path::new("dictionaries").exists() {
        if let Err(e) = std::fs::create_dir("dictionaries") {
            println!("\x1b[31m❌ Impossible de créer le dossier dictionaries: {e}\x1b[0m");
            return None;
        }
    }

    let dest = "dictionaries/rockyou.txt";
    println!("\x1b[33mTéléchargement du dictionnaire rockyou.txt (14 millions de mots de passe)...\x1b[0m");
    println!("\x1b[36mSource: https://github.com/brannondorsey/naive-hashcat\x1b[0m");
    match reqwest::blocking::get(url) {
        Ok(resp) => {
            let mut out = std::fs::File::create(dest).ok()?;
            let content = resp.bytes().ok()?;
            std::io::copy(&mut content.as_ref(), &mut out).ok()?;
            println!("\x1b[32m✅ Dictionnaire rockyou.txt téléchargé avec succès !\x1b[0m");
            println!("\x1b[36m📁 Fichier sauvegardé: {dest}\x1b[0m");
            Some(dest.to_string())
        }
        Err(e) => {
            println!("\x1b[31m❌ Échec du téléchargement du dictionnaire.\x1b[0m");
            println!("\x1b[33mErreur: {e}\x1b[0m");
            println!("\x1b[36m💡 Vous pouvez télécharger manuellement rockyou.txt et le placer dans le dossier dictionaries/.\x1b[0m");
            None
        }
    }
}

fn main() {
    let args = Args::parse();

    // Demande à l'utilisateur de fournir le mot de passe haché à brute-forcer
    let target_password_hash = args.hash.unwrap_or_else(|| {
        config::read_line("Veuillez entrer le mot de passe haché à brute-forcer :")
    });

    if target_password_hash.is_empty() {
        eprintln!("\x1b[31m❌ Hash vide. Utilisez --hash ou entrez un hash valide.\x1b[0m");
        std::process::exit(1);
    }

    // Demande le salt
    let salt = if args.non_interactive {
        String::new()
    } else {
        config::read_line("Le hash utilise-t-il un salt connu ? (laisser vide si non)")
    };

    let salt_position = if !salt.is_empty() && !args.non_interactive {
        config::ask_salt_position()
    } else {
        hashing::SaltPosition::After
    };

    // Détection automatique des dictionnaires
    let dictionaries = config::detect_dictionaries();
    let dictionary_path = args.dictionary.or_else(|| {
        if dictionaries.is_empty() {
            println!("\x1b[31m❌ Aucun dictionnaire trouvé dans ./, ./dictionaries/ ou ./assets/.\x1b[0m");
            println!("\x1b[36m💡 Le programme peut télécharger automatiquement un dictionnaire populaire.\x1b[0m");
            if config::read_yes_no("Voulez-vous télécharger un dictionnaire populaire (rockyou.txt) ? (o/n) [O]", true) {
                download_rockyou()
            } else {
                None
            }
        } else {
            println!("\x1b[34m📚 Dictionnaires détectés :\x1b[0m");
            for (i, dict) in dictionaries.iter().enumerate() {
                println!("  [{}] {}", i + 1, dict);
            }
            println!("  [{}] 📥 Télécharger rockyou.txt", dictionaries.len() + 1);
            println!("  [{}] ❌ Ne pas utiliser de dictionnaire", dictionaries.len() + 2);
            let choice = config::read_line("\nVeuillez choisir une option (numéro) :");
            if let Ok(idx) = choice.parse::<usize>() {
                if idx > 0 && idx <= dictionaries.len() {
                    Some(dictionaries[idx - 1].clone())
                } else if idx == dictionaries.len() + 1 {
                    download_rockyou()
                } else {
                    None
                }
            } else {
                None
            }
        }
    });

    // Demande le mode streaming AVANT de charger (uniquement pour les dictionnaires .txt)
    let (use_streaming, streaming_path, dictionary) = if let Some(path) = dictionary_path.as_ref() {
        if path.ends_with(".zip") {
            // Les ZIP sont toujours chargés en mémoire
            let dict = Some(load_zipped_dictionary(path));
            (false, String::new(), dict)
        } else {
            // Pour les .txt : proposer le streaming avant de charger
            println!("Votre dictionnaire est-il trop volumineux pour être chargé en mémoire ?");
            println!("Mode streaming recommandé pour les fichiers >100MB");
            let use_streaming = config::read_yes_no("Utiliser le mode streaming ? (o/n) [N]", false);
            if use_streaming {
                println!("\x1b[33m⚠️  Mode streaming activé\x1b[0m");
                (true, path.clone(), None)
            } else {
                match fs::read_to_string(path) {
                    Ok(content) => (
                        false,
                        String::new(),
                        Some(content.lines().map(|l| l.to_string()).collect()),
                    ),
                    Err(_) => {
                        println!("\x1b[33m⚠️  Le fichier contient des caractères non-UTF8, tentative de conversion...\x1b[0m");
                        match fs::read(path) {
                            Ok(bytes) => {
                                let content = String::from_utf8_lossy(&bytes);
                                (
                                    false,
                                    String::new(),
                                    Some(content.lines().map(|l| l.to_string()).collect()),
                                )
                            }
                            Err(e) => {
                                println!("\x1b[31m❌ Erreur lors de la lecture du fichier: {e}\x1b[0m");
                                (false, String::new(), None)
                            }
                        }
                    }
                }
            }
        }
    } else {
        (false, String::new(), None)
    };

    // Tentative de détection automatique de l'algorithme de hachage
    let algorithm = args.algorithm.unwrap_or_else(|| {
        match hashing::detect_algorithm(&target_password_hash) {
            Ok(algo) => {
                println!("Algorithme détecté : {algo}");
                algo
            }
            Err(err) => {
                config::read_line(&format!(
                    "Erreur de détection : {err}. Spécifiez l'algorithme (md5, sha1, sha256, sha512, bcrypt, argon2, base64) :"
                ))
            }
        }
    });

    // Charset : personnaliser seulement si pas de dictionnaire
    let charset = if dictionary.is_none() {
        config::build_charset()
    } else {
        String::new()
    };

    let num_threads = args.threads.unwrap_or_else(config::ask_num_threads);

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
    let spinner_handle =
        telemetry::start_spinner_thread(Arc::clone(&is_running), Arc::clone(&total_attempts));

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
        salt,
        salt_position,
    );

    // Stop telemetry and spinner once the brute-force process ends
    let duration = start.elapsed();
    *is_running.lock().unwrap() = false;
    telemetry_handle.join().unwrap();
    spinner_handle.join().unwrap();

    // Display results
    let total = *total_attempts.lock().unwrap();
    let _per_sec = *attempts_per_second.lock().unwrap();
    println!(
        "\x1b[32m\nRecherche complétée en {duration:?} secondes avec {total} tentatives\x1b[0m"
    );
    println!("\x1b[1;33mSi le mot de passe a été trouvé, il est affiché ci-dessus.\x1b[0m");
}
