use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(clippy::too_many_arguments)]
pub fn start_brute_force(
    charset: &str,
    target_password_hash: &str,
    algorithm: &str,
    total_attempts: Arc<Mutex<u64>>,
    attempts_per_second: Arc<Mutex<u64>>,
    is_running: Arc<Mutex<bool>>,
    dictionary: Option<Vec<String>>,
    use_streaming: bool,
    streaming_path: String,
    num_threads: usize,
) {
    if use_streaming {
        let mut handles = Vec::new();
        let found_flag = Arc::clone(&is_running);
        for thread_id in 0..num_threads {
            let path = streaming_path.clone();
            let target_password_hash = target_password_hash.to_string();
            let algorithm = algorithm.to_string();
            let total_attempts = Arc::clone(&total_attempts);
            let attempts_per_second = Arc::clone(&attempts_per_second);
            let found_flag = Arc::clone(&found_flag);
            let handle = thread::spawn(move || {
                for (i, word) in iter_dictionary_file(&path).enumerate() {
                    if i % num_threads != thread_id {
                        continue;
                    }
                    if !*found_flag.lock().unwrap() {
                        break;
                    }
                    let attempt_hash = crate::hashing::hash_password(&word, &algorithm);
                    {
                        let mut total = total_attempts.lock().unwrap();
                        *total += 1;
                        let mut attempts_per_sec = attempts_per_second.lock().unwrap();
                        *attempts_per_sec += 1;
                    }
                    if attempt_hash == target_password_hash {
                        println!("\n\x1b[1;32mMot de passe trouvé : {word}\x1b[0m");
                        *found_flag.lock().unwrap() = false;
                        break;
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        *is_running.lock().unwrap() = false;
        return;
    }

    if let Some(dict) = dictionary {
        if !dict.is_empty() {
            let chunk_size = dict.len().div_ceil(num_threads);
            let mut handles = Vec::new();
            let found_flag = Arc::clone(&is_running);
            for chunk in dict.chunks(chunk_size) {
                let chunk = chunk.to_owned();
                let target_password_hash = target_password_hash.to_string();
                let algorithm = algorithm.to_string();
                let total_attempts = Arc::clone(&total_attempts);
                let attempts_per_second = Arc::clone(&attempts_per_second);
                let found_flag = Arc::clone(&found_flag);
                let handle = thread::spawn(move || {
                    for word in chunk {
                        if !*found_flag.lock().unwrap() {
                            break;
                        }
                        let word_str = String::from_utf8_lossy(word.as_bytes()).to_string();
                        let attempt_hash = crate::hashing::hash_password(&word_str, &algorithm);
                        {
                            let mut total = total_attempts.lock().unwrap();
                            *total += 1;
                            let mut attempts_per_sec = attempts_per_second.lock().unwrap();
                            *attempts_per_sec += 1;
                        }
                        if attempt_hash == target_password_hash {
                            println!("\n\x1b[1;32mMot de passe trouvé : {word_str}\x1b[0m");
                            *found_flag.lock().unwrap() = false;
                            break;
                        }
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                let _ = handle.join();
            }
            *is_running.lock().unwrap() = false;
            return;
        }
    }

    // If no dictionary, use brute-force character generation
    let charset_vec: Vec<char> = charset.chars().collect();
    let mut handles = Vec::new();
    let found_flag = Arc::clone(&is_running);
    for thread_id in 0..num_threads {
        let charset_vec = charset_vec.clone();
        let target_password_hash = target_password_hash.to_string();
        let algorithm = algorithm.to_string();
        let total_attempts = Arc::clone(&total_attempts);
        let attempts_per_second = Arc::clone(&attempts_per_second);
        let found_flag = Arc::clone(&found_flag);
        let handle = thread::spawn(move || {
            for length in 1.. {
                let iter = if length == 1 {
                    Box::new(charset_vec.iter().enumerate().filter_map(move |(i, &c)| {
                        if i % num_threads == thread_id {
                            Some(c.to_string())
                        } else {
                            None
                        }
                    })) as Box<dyn Iterator<Item = String>>
                } else {
                    Box::new(generate_combinations_iter_with_prefix(
                        charset_vec.clone(),
                        length,
                        thread_id,
                        num_threads,
                    )) as Box<dyn Iterator<Item = String>>
                };
                for attempt in iter {
                    if !*found_flag.lock().unwrap() {
                        break;
                    }
                    let attempt_hash = crate::hashing::hash_password(&attempt, &algorithm);
                    {
                        let mut total = total_attempts.lock().unwrap();
                        *total += 1;
                        let mut attempts_per_sec = attempts_per_second.lock().unwrap();
                        *attempts_per_sec += 1;
                    }
                    if attempt_hash == target_password_hash {
                        println!("\n\x1b[1;32mMot de passe trouvé : {attempt}\x1b[0m");
                        *found_flag.lock().unwrap() = false;
                        break;
                    }
                }
                if !*found_flag.lock().unwrap() {
                    break;
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    *is_running.lock().unwrap() = false;
}

// Génère les combinaisons de longueur 'length' commençant par les préfixes attribués à ce thread
pub fn generate_combinations_iter_with_prefix(
    charset_vec: Vec<char>,
    length: usize,
    thread_id: usize,
    num_threads: usize,
) -> Box<dyn Iterator<Item = String>> {
    let charset_len = charset_vec.len();
    Box::new((0..charset_len.pow(length as u32)).filter_map(move |i| {
        // On ne garde que les indices qui correspondent à ce thread
        if i % num_threads != thread_id {
            return None;
        }
        let mut result = String::with_capacity(length);
        let mut num = i;
        for _ in 0..length {
            result.push(charset_vec[num % charset_len]);
            num /= charset_len;
        }
        Some(result)
    }))
}

pub fn iter_dictionary_file(path: &str) -> Box<dyn Iterator<Item = String>> {
    let file = std::fs::File::open(path).expect("Impossible d'ouvrir le fichier dictionnaire");
    let reader = std::io::BufReader::new(file);
    Box::new(reader.lines().map_while(Result::ok))
}
