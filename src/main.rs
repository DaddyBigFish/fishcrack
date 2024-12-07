use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs;
use std::env;
use std::process;
use std::path::Path;
use home::home_dir;
use instant::Instant;
use lazy_static::lazy_static;

mod algorithms;
mod banner;

lazy_static! {
    static ref HASH_NAME: String = env::args().nth(2).unwrap();
    static ref HASH_INPUT: String = env::args().nth(3).unwrap();
    static ref LOCAL_HASH_PATH: String =
        format!("{}/.fishcrack/hashes.saved", home_dir().unwrap().display());
}

static mut LOAD_TIME: u128 = 0;

#[derive(Serialize, Deserialize, Clone)]
struct LocalHash {
    hash: String,
    plaintext: String,
}

fn load_local_hive() -> Vec<LocalHash> {
    let json_file_path = Path::new(&*LOCAL_HASH_PATH);
    let data = fs::read_to_string(json_file_path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}

fn crack(line: &str, hash_type: &str, start_time: Instant, cracked: &Arc<AtomicBool>) -> Option<LocalHash> {
    let formatted_hash = algorithms::create_hash(line, hash_type);
    if formatted_hash == *HASH_INPUT {
        unsafe {
            println!(
                "🤍 Cracked! {} -> \"{}\" in {} millisecs",
                formatted_hash,
                line,
                start_time.elapsed().as_millis() - LOAD_TIME
            );
        }
        cracked.store(true, Ordering::SeqCst);
        Some(LocalHash { hash: formatted_hash, plaintext: line.to_string() })
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ThreadPoolBuilder::new().num_threads(64).build_global().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    banner::display_banner();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: fishcrack <wordlist> [--spray | <hash_name>] <hash_value>");
        process::exit(1);
    }

    let wordlist_file = &args[1];
    let is_spray_mode = args.contains(&String::from("--spray"));

    fs::create_dir_all(format!("{}/.fishcrack", home_dir().unwrap().display()))?;
    let f = fs::File::open(&*LOCAL_HASH_PATH);
    if f.is_err() {
        fs::File::create(&*LOCAL_HASH_PATH)?;
    }

    let local_hive = load_local_hive();
    if let Some(hash) = local_hive.iter().find(|h| h.hash == *HASH_INPUT) {
        println!("🤍 Saved hash found! {} -> \"{}\"", hash.hash, hash.plaintext);
        process::exit(0);
    }

    let valid_hashes = vec![
        "MD2", "MD4", "MD5", "SHA-1", "RipeMD-128", "RipeMD-160", "RipeMD-256", "RipeMD-320",
        "SHA-224", "SHA-256", "SHA-384", "SHA-512", "SHA3-224", "SHA3-256", "SHA3-384", "SHA3-512",
    ];

    let start_time = Instant::now();
    let file_content = fs::read_to_string(wordlist_file)?;
    let dict: Vec<&str> = file_content.lines().collect();

    unsafe {
        LOAD_TIME = start_time.elapsed().as_millis();
        println!("Loaded the wordlist file in {} millisecs.", LOAD_TIME);
    }

    let results = Arc::new(RwLock::new(Vec::new()));
    let cracked = Arc::new(AtomicBool::new(false));

    if is_spray_mode {
        println!("Now spraying! Attempting all hash algorithms.");
        valid_hashes.par_iter().flat_map(|hash_type| {
            dict.par_iter().map(move |line| (hash_type, line))
        }).find_any(|(hash_type, line)| {
            if !cracked.load(Ordering::SeqCst) {
                if let Some(result) = crack(line, hash_type, start_time, &cracked) {
                    let mut res = results.write().unwrap();
                    res.push(result);
                    return true;
                }
            }
            false
        });
    } else {
        let hash_type = HASH_NAME.as_str();
        if !valid_hashes.contains(&hash_type) {
            println!("❌ Invalid hash name!");
            process::exit(1);
        }

        dict.par_iter().find_any(|line| {
            if !cracked.load(Ordering::SeqCst) {
                if let Some(result) = crack(line, hash_type, start_time, &cracked) {
                    let mut res = results.write().unwrap();
                    res.push(result);
                    return true;
                }
            }
            false
        });
    }

    if cracked.load(Ordering::SeqCst) {
        fs::write(&*LOCAL_HASH_PATH, serde_json::to_string(&*results.read().unwrap()).unwrap()).unwrap();
    }

    Ok(())
}
