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
                "💛 Cracked! That took {} millisecs.\n\
                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                {:>22}🔑 {}\n\
                \n\
                ",
                start_time.elapsed().as_millis() - LOAD_TIME,
                "",
                line
            );
        }
        cracked.store(true, Ordering::SeqCst);
        Some(LocalHash { hash: formatted_hash, plaintext: line.to_string() })
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ThreadPoolBuilder::new().num_threads(50).build_global().map_err(|e| {
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

    // Define valid hash lengths for filtering
    let valid_hash_lengths = vec![
        (32, vec!["MD2", "MD4", "MD5", "MD6-128", "RIPEMD-128"]),
        (40, vec!["RIPEMD-160", "SHA-1"]),
        (56, vec!["SHA-224", "SHA3-224"]),
        (64, vec!["MD6-256", "RIPEMD-256", "SHA-256", "SHA3-256"]),
        (80, vec!["RIPEMD-320"]),
        (96, vec!["SHA-384", "SHA3-384"]),
        (128, vec!["MD6-512", "SHA-512", "SHA3-512"]),
    ];

    let start_time = Instant::now();
    let file_content = fs::read(wordlist_file)?;
    let valid_content = String::from_utf8_lossy(&file_content);
    let dict: Vec<&str> = valid_content.lines().collect();

    unsafe {
        LOAD_TIME = start_time.elapsed().as_millis();
        println!("⚫ Loaded the wordlist file in {} millisecs.", LOAD_TIME);
    }

    let results = Arc::new(RwLock::new(Vec::new()));
    let cracked = Arc::new(AtomicBool::new(false));

    if is_spray_mode {
        println!("⚫ Now spraying! Attempting all hash algorithms.");

        let input_hash_length = HASH_INPUT.len();
        let valid_hashes = valid_hash_lengths
            .iter()
            .find(|(length, _)| *length == input_hash_length)
            .map(|(_, hashes)| hashes.clone())
            .unwrap_or_else(Vec::new);

        if valid_hashes.is_empty() {
            println!("❌ No hash algorithms found matching length {}!", input_hash_length);
            process::exit(1);
        }

        println!("⚫ Attempting length {}: {:?}", input_hash_length, valid_hashes);

        valid_hashes.par_iter().for_each(|hash_type| {
            dict.par_iter().for_each(|line| {
                if !cracked.load(Ordering::SeqCst) {
                    if let Some(result) = crack(line, hash_type, start_time, &cracked) {
                        let mut res = results.write().unwrap();
                        res.push(result);
                    }
                }
            });
        });
    } else {
        let hash_type = HASH_NAME.as_str();
        if !valid_hash_lengths.iter().any(|(_, hashes)| hashes.contains(&hash_type)) {
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
