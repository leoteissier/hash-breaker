use std::fs::File;
use std::io::Read;
use zip::read::ZipArchive;

/// Fonction pour charger un fichier texte depuis une archive ZIP sur le disque
pub fn load_zipped_dictionary(zip_path: &str) -> Vec<String> {
    // Try to open the ZIP file
    let file = match File::open(zip_path) {
        Ok(file) => file,
        Err(err) => panic!("Unable to open the ZIP file: {}", err),
    };

    // Try to read the ZIP archive
    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => panic!("Unable to read the ZIP archive: {}", err),
    };

    // Chercher le premier fichier texte dans le ZIP
    let mut file_in_zip = None;
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            let name = file.name();
            if name.ends_with(".txt") || name.ends_with(".lst") || name.ends_with(".dict") {
                file_in_zip = Some(name.to_string());
                break;
            }
        }
    }

    let file_in_zip = file_in_zip.unwrap_or_else(|| {
        // Si aucun fichier texte trouvé, essayer les noms courants
        let common_names = ["passwords.txt", "rockyou.txt", "wordlist.txt", "dictionary.txt"];
        for name in &common_names {
            if archive.by_name(name).is_ok() {
                return name.to_string();
            }
        }
        panic!("Aucun fichier texte trouvé dans le ZIP. Fichiers disponibles: {:?}", 
               (0..archive.len()).filter_map(|i| archive.by_index(i).ok().map(|f| f.name().to_string())).collect::<Vec<_>>());
    });

    println!("\x1b[36m📁 Lecture du fichier '{}' dans le ZIP\x1b[0m", file_in_zip);

    // Try to locate the file inside the ZIP archive
    let mut file = match archive.by_name(&file_in_zip) {
        Ok(file) => file,
        Err(err) => panic!("Unable to find '{}' in ZIP archive: {}", file_in_zip, err),
    };

    let mut contents = Vec::new();
    // Try to read the file contents
    if let Err(err) = file.read_to_end(&mut contents) {
        panic!("Error reading the file: {}", err);
    }

    // Convert the byte contents to a String
    let contents_str = String::from_utf8_lossy(&contents).to_string();

    // Split lines into Vec<String>
    contents_str.lines().map(|line| line.to_string()).collect()
}





