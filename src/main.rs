use std::iter::Iterator;
use std::time::{Instant, Duration};
use std::io::{self, Write, stdin};
use itertools::Itertools;
use std::{thread, sync::{Arc, Mutex}};

fn main() {
    println!("Veuillez entrer le mot de passe à brute-forcer:");
    let mut target_password = String::new();
    stdin().read_line(&mut target_password).unwrap();
    target_password = target_password.trim().to_string();

    let charset = "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?";
    let max_length = target_password.len();

    let is_running = Arc::new(Mutex::new(true));
    let attempts_per_second = Arc::new(Mutex::new(0));
    let total_attempts = Arc::new(Mutex::new(0));

    let attempts_per_second_clone = Arc::clone(&attempts_per_second);
    let is_running_clone = Arc::clone(&is_running);

    let telemetry_handle = thread::spawn(move || {
        let mut last_print = Instant::now();
        while *is_running_clone.lock().unwrap() {
            if last_print.elapsed() >= Duration::from_secs(1) {
                let attempts = {
                    let mut num = attempts_per_second_clone.lock().unwrap();
                    let val = *num;
                    *num = 0;
                    val
                };
                print!("\rTentatives par seconde: {}          ", attempts);
                io::stdout().flush().unwrap();
                last_print = Instant::now();
            }
        }
    });

    let is_running_handle = Arc::clone(&is_running);
    let total_attempts_handle = Arc::clone(&total_attempts);
    let handle = thread::spawn(move || {
        let spinner = ['|', '/', '-', '\\'];
        let mut index = 0;
        while *is_running_handle.lock().unwrap() {
            let total = *total_attempts_handle.lock().unwrap();
            print!("\r{} Recherche en cours - Tentatives: {}", spinner[index], total);
            io::stdout().flush().unwrap();
            index = (index + 1) % spinner.len();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let start = Instant::now();

    for length in 1..=max_length {
        let combinations = charset.chars().permutations(length).unique();
        for combination in combinations {
            let attempt = combination.iter().collect::<String>();
            {
                let mut total = total_attempts.lock().unwrap();
                *total += 1;
            }

            if attempt == target_password {
                let duration = start.elapsed();
                *is_running.lock().unwrap() = false;
                handle.join().unwrap();
                telemetry_handle.join().unwrap();
                let total = *total_attempts.lock().unwrap();
                println!("\rMot de passe trouvé: {} en {:?} secondes, avec {} tentatives", attempt, duration, total);
                return;
            }
        }
    }

    let duration = start.elapsed();
    *is_running.lock().unwrap() = false;
    handle.join().unwrap();
    telemetry_handle.join().unwrap();
    let total = *total_attempts.lock().unwrap();
    println!("\rMot de passe non trouvé, recherche complétée en {:?} secondes avec {} tentatives", duration, total);
}