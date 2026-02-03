//! Configuration interactive et utilitaires pour le brute-force.

use crate::hashing::SaltPosition;
use std::fs;
use std::io::stdin;

const DEFAULT_CHARSET: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?";

/// Lit une ligne depuis stdin et retourne le contenu trimé.
pub fn read_line(prompt: &str) -> String {
    if !prompt.is_empty() {
        println!("{prompt}");
    }
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    input.trim().to_string()
}

/// Lit une ligne et retourne true si l'utilisateur répond oui (o, oui, ou vide pour défaut).
pub fn read_yes_no(prompt: &str, default_yes: bool) -> bool {
    let input = read_line(prompt).to_lowercase();
    if input.is_empty() {
        return default_yes;
    }
    input == "o" || input == "oui" || input == "y" || input == "yes"
}

/// Détecte les dictionnaires dans les répertoires standards.
pub fn detect_dictionaries() -> Vec<String> {
    let mut dicts = Vec::new();
    for dir in &[".", "./dictionaries", "./assets"] {
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

/// Construit le charset personnalisé selon les choix de l'utilisateur.
pub fn build_charset() -> String {
    if !read_yes_no(
        "Voulez-vous personnaliser le jeu de caractères utilisé pour le brute-force ? (o/n) [N]",
        false,
    ) {
        return DEFAULT_CHARSET.to_string();
    }

    let chiffres = read_yes_no("Inclure les chiffres ? (o/n) [O]", true);
    let minuscules = read_yes_no("Inclure les minuscules ? (o/n) [O]", true);
    let majuscules = read_yes_no("Inclure les majuscules ? (o/n) [O]", true);
    let symboles = read_yes_no("Inclure les symboles spéciaux ? (o/n) [N]", false);

    let mut cs = String::new();
    if chiffres {
        cs.push_str("0123456789");
    }
    if minuscules {
        cs.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if majuscules {
        cs.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if symboles {
        cs.push_str("!@#$%^&*()_+-=[]{}|;:',.<>/?");
    }

    if cs.is_empty() {
        println!("Aucun jeu de caractères sélectionné, utilisation du jeu complet par défaut.");
        DEFAULT_CHARSET.to_string()
    } else {
        cs
    }
}

/// Demande la position du salt (avant/après).
pub fn ask_salt_position() -> SaltPosition {
    let pos = read_line("Le salt est-il AVANT ou APRÈS le mot de passe ? (avant/après) [après]")
        .to_lowercase();
    if pos.starts_with("av") {
        SaltPosition::Before
    } else {
        SaltPosition::After
    }
}

/// Demande le nombre de threads à utiliser.
pub fn ask_num_threads() -> usize {
    let total_cores = num_cpus::get();
    println!("Votre machine possède {total_cores} cœurs logiques.");
    if read_yes_no("Voulez-vous utiliser tous les cœurs disponibles ? (o/n) [O]", true) {
        return total_cores;
    }
    let input = read_line(&format!("Combien de cœurs souhaitez-vous utiliser ? (1-{total_cores})"));
    let n = input.parse::<usize>().unwrap_or(1);
    if n > 0 && n <= total_cores {
        n
    } else {
        1
    }
}
