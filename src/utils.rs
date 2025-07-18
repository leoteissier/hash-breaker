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

    // Specify the file name within the ZIP (this can be adjusted as needed)
    let file_in_zip = "passwords.txt";

    // Try to locate the file inside the ZIP archive
    let mut file = match archive.by_name(file_in_zip) {
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





