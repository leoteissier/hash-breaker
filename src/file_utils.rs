use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use zip::read::ZipArchive;

/// Fonction pour charger un fichier texte depuis une archive ZIP sur le disque
pub fn load_zipped_dictionary(zip_path: &str, file_in_zip: &str) -> Vec<String> {
    let file = File::open(zip_path).expect("Impossible d'ouvrir le fichier ZIP");
    let mut archive = ZipArchive::new(file).expect("Impossible de lire l'archive ZIP");

    // Ouvrir le fichier texte à l'intérieur de l'archive ZIP
    let mut file = archive.by_name(file_in_zip).expect("Impossible de trouver le fichier dans le ZIP");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Erreur lors de la lecture du fichier compressé");

    // Retourner les lignes du fichier sous forme de Vec<String>
    contents.lines().map(|line| line.trim().to_string()).collect()
}

/// Fonction pour charger un fichier texte depuis un ZIP intégré dans le binaire
pub fn load_zipped_dictionary_from_embedded() -> Vec<String> {
    // Charger le fichier ZIP intégré
    let zip_bytes = include_bytes!("../assets/passwords.zip");

    // Créer un curseur pour lire les octets
    let reader = Cursor::new(zip_bytes);

    // Ouvrir l'archive ZIP
    let mut archive = ZipArchive::new(reader).expect("Impossible de lire l'archive ZIP");

    // Ouvrir le fichier texte à l'intérieur de l'archive ZIP
    let mut file = archive.by_name("passwords.txt").expect("Impossible de trouver le fichier dans le ZIP");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Erreur lors de la lecture du fichier compressé");

    // Retourner les lignes du fichier sous forme de Vec<String>
    contents.lines().map(|line| line.trim().to_string()).collect()
}
