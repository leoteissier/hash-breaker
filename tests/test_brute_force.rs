use hash_breaker::brute_force;
use hash_breaker::hashing::hash_password;
use std::sync::{Arc, Mutex};

#[test]
fn test_brute_force_found_with_dictionary() {
    let charset = ""; // Pas besoin de charset puisque nous utilisons un dictionnaire
    let target_password = "abc";
    let algorithm = "md5"; // Algorithme choisi (assurez-vous qu'il est pris en charge)

    // Calculer le hachage du mot de passe cible
    let target_password_hash = hash_password(target_password, algorithm);

    // Créer un petit dictionnaire avec le mot de passe cible inclus
    let dictionary = vec![
        "password123".to_string(),
        "letmein".to_string(),
        target_password.to_string(), // Le mot de passe cible
        "123456".to_string(),
    ];

    let total_attempts = Arc::new(Mutex::new(0u64));
    let attempts_per_second = Arc::new(Mutex::new(0u64));
    let is_running = Arc::new(Mutex::new(true));

    // Appeler la fonction brute_force avec le dictionnaire
    brute_force::start_brute_force(
        charset,
        &target_password_hash,
        algorithm,
        Arc::clone(&total_attempts),
        Arc::clone(&attempts_per_second),
        Arc::clone(&is_running),
        Some(dictionary.clone()), // Passer le dictionnaire
        false,                    // use_streaming
        String::new(),            // streaming_path
        1,                        // num_threads
    );

    // Vérifier que la recherche s'est bien arrêtée
    assert_eq!(*is_running.lock().unwrap(), false);

    // Vérifier que le nombre de tentatives correspond à la position du mot de passe dans le dictionnaire
    let attempts = *total_attempts.lock().unwrap();
    assert_eq!(
        attempts, 3,
        "Le nombre de tentatives doit être égal à 3, mais était {}",
        attempts
    );
}

#[test]
fn test_brute_force_not_found_with_dictionary() {
    let charset = ""; // Pas besoin de charset puisque nous utilisons un dictionnaire
    let target_password = "xyz"; // Un mot de passe qui n'est pas dans le dictionnaire
    let algorithm = "md5"; // Algorithme choisi

    // Calculer le hachage du mot de passe cible
    let target_password_hash = hash_password(target_password, algorithm);

    // Créer un dictionnaire avec des mots de passe courts
    let dictionary = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];

    let total_attempts = Arc::new(Mutex::new(0u64));
    let attempts_per_second = Arc::new(Mutex::new(0u64));
    let is_running = Arc::new(Mutex::new(true));

    // Appeler la fonction brute_force avec le dictionnaire
    brute_force::start_brute_force(
        charset,
        &target_password_hash,
        algorithm,
        Arc::clone(&total_attempts),
        Arc::clone(&attempts_per_second),
        Arc::clone(&is_running),
        Some(dictionary.clone()), // Passer le dictionnaire
        false,                    // use_streaming
        String::new(),            // streaming_path
        1,                        // num_threads
    );

    // Vérifier que la recherche s'est bien arrêtée
    assert_eq!(*is_running.lock().unwrap(), false);

    // Vérifier que le nombre de tentatives correspond à la taille du dictionnaire
    let attempts = *total_attempts.lock().unwrap();
    assert_eq!(
        attempts,
        dictionary.len() as u64,
        "Le nombre de tentatives doit être égal à la taille du dictionnaire"
    );
}

#[test]
fn test_brute_force_found_without_dictionary() {
    let charset = "abc"; // Charset utilisé pour les combinaisons
    let target_password = "abc";
    let algorithm = "md5"; // Algorithme choisi (assurez-vous qu'il est pris en charge)

    // Calculer le hachage du mot de passe cible
    let target_password_hash = hash_password(target_password, algorithm);

    let total_attempts = Arc::new(Mutex::new(0u64));
    let attempts_per_second = Arc::new(Mutex::new(0u64));
    let is_running = Arc::new(Mutex::new(true));

    // Appeler la fonction brute_force sans dictionnaire, il doit générer toutes les combinaisons
    brute_force::start_brute_force(
        charset,
        &target_password_hash,
        algorithm,
        Arc::clone(&total_attempts),
        Arc::clone(&attempts_per_second),
        Arc::clone(&is_running),
        None,          // Pas de dictionnaire
        false,         // use_streaming
        String::new(), // streaming_path
        1,             // num_threads
    );

    // Vérifier que la recherche s'est bien arrêtée
    assert_eq!(*is_running.lock().unwrap(), false);

    // Le mot de passe "abc" sera trouvé au bout d'un certain nombre de combinaisons
    let attempts = *total_attempts.lock().unwrap();
    assert!(
        attempts >= 1,
        "Le nombre de tentatives doit être supérieur ou égal à 1"
    );
}
