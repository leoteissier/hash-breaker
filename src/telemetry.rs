use std::sync::{Arc, Mutex};
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


