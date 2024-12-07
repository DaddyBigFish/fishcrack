use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
use md2::Md2;
use md4::Md4;

pub fn create_hash(line: &str, hash_name: &str) -> String {
    match hash_name {
        "MD2" => {
            let mut hasher = Md2::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }
    
        "MD4" => {
            let mut hasher = Md4::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        "MD5" => {
            let hashvalue = md5::Md5::digest(line.as_bytes());
            format!("{:x}", hashvalue)
        }

        "SHA-1" => {
            let mut hasher = Sha1::new();
            hasher.update(line.as_bytes());
            format!("{}", hasher.digest().to_string())
        }

        "RipeMD-128" => {
            let result = Ripemd128::digest(line.as_bytes());
            format!("{:x}", result)
        }
        
        "RipeMD-160" => {
            let result = Ripemd160::digest(line.as_bytes());
            format!("{:x}", result)
        }

        "RipeMD-256" => {
            let result = Ripemd256::digest(line.as_bytes());
            format!("{:x}", result)
        }
        
        "RipeMD-320" => {
            let result = Ripemd320::digest(line.as_bytes());
            format!("{:x}", result)
        }

        "SHA-224" => {
            let mut hasher = Sha224::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        "SHA-256" => {
            let mut hasher = Sha256::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        "SHA-384" => {
            let mut hasher = Sha384::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        "SHA-512" => {
            let mut hasher = Sha512::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        "SHA3-224" => {
            let mut hasher = Sha3_224::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "SHA3-256" => {
            let mut hasher = Sha3_256::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "SHA3-384" => {
            let mut hasher = Sha3_384::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "SHA3-512" => {
            let mut hasher = Sha3_512::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        _ => "".to_string(),
    }
}
