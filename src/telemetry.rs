use std::sync::{Arc, Mutex, mpsc};
use std::time::{Instant, Duration};
use std::io::{self, Write};
use std::thread;

pub fn start_telemetry_thread(
    is_running: Arc<Mutex<bool>>, 
    total_attempts: Arc<Mutex<u64>>,
    attempts_per_second: Arc<Mutex<u64>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut last_print = Instant::now();
        while *is_running.lock().unwrap() {
            if last_print.elapsed() >= Duration::from_secs(1) {
                let attempts = {
                    let mut num = attempts_per_second.lock().unwrap();
                    let val = *num;
                    *num = 0;  // Reset the counter for attempts per second
                    val
                };
                let total = *total_attempts.lock().unwrap();
                print!("\rRecherche en cours - Tentatives: {} | Tentatives par seconde: {}", total, attempts);
                io::stdout().flush().unwrap();
                last_print = Instant::now();
            }
        }
    })
}

pub fn start_spinner_thread(
    is_running: Arc<Mutex<bool>>, 
    total_attempts: Arc<Mutex<u64>>
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let spinner = ['|', '/', '-', '\\'];
        let mut index = 0;
        while *is_running.lock().unwrap() {
            let total = *total_attempts.lock().unwrap();

            // Print spinner and total attempts on the same line
            print!("\r{} Recherche en cours - Tentatives: {}", spinner[index], total);
            io::stdout().flush().unwrap();
            
            index = (index + 1) % spinner.len();
            thread::sleep(Duration::from_millis(100));  // Update spinner every 100ms
        }
    })
}

// Nouvelle fonction pour démarrer la télémétrie asynchrone via un channel
pub fn start_telemetry_channel_thread(
    is_running: Arc<Mutex<bool>>,
    rx: mpsc::Receiver<(u64, u64)>,
) -> std::thread::JoinHandle<()> {
    use std::time::Instant;
    use std::io::{self, Write};
    std::thread::spawn(move || {
        let mut last_print = Instant::now();
        let mut total = 0u64;
        let mut attempts = 0u64;
        while *is_running.lock().unwrap() {
            // On attend les mises à jour du channel
            if let Ok((t, a)) = rx.try_recv() {
                total = t;
                attempts = a;
            }
            if last_print.elapsed() >= std::time::Duration::from_secs(1) {
                print!("\rRecherche en cours - Tentatives: {} | Tentatives par seconde: {}", total, attempts);
                io::stdout().flush().unwrap();
                last_print = Instant::now();
            }
        }
    })
}
