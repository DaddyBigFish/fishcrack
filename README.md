## Introduction
![81-813291_22294-tropical-fish-icon-yellow-fish-emoji (1)](https://github.com/user-attachments/assets/f86f9c36-560c-4664-a253-ee60c7aad136)    

𝗳𝗶𝘀𝗵𝗰𝗿𝗮𝗰𝗸 the hash cracker written in Rust.

## Supported hashes
| Name        | Algorithm  | Crate     | Version   | Crates.io |
|-------------|------------|-----------|-----------|-----------|
| `MD2`       | MD2        | md2       | 0.10.2    | https://crates.io/crates/md2/0.10.2 |
| `MD4`       | MD4        | md4       | 0.10.2    | https://crates.io/crates/md4/0.10.2 |
| `MD5`       | MD5        | md-5      | 0.10.4    | https://crates.io/crates/md-5/0.10.4 |
| `SHA-1`     | SHA-1      | sha1      | 0.6.0     | https://crates.io/crates/sha1/0.6.0 |
| `SHA-224`   | SHA-2 224  | sha2      | 0.10.6    | https://crates.io/crates/sha2/0.10.6 |
| `SHA-256`   | SHA-2 256  | sha2      | 0.10.6    | https://crates.io/crates/sha2/0.10.6 |
| `SHA-384`   | SHA-2 384  | sha2      | 0.10.6    | https://crates.io/crates/sha2/0.10.6 |
| `SHA-512`   | SHA-2 512  | sha2      | 0.10.6    | https://crates.io/crates/sha2/0.10.6 |
| `SHA3-224`  | SHA-3 224  | sha3      | 0.10.6    | https://crates.io/crates/sha3/0.10.6 |
| `SHA3-256`  | SHA-3 256  | sha3      | 0.10.6    | https://crates.io/crates/sha3/0.10.6 |
| `SHA3-384`  | SHA-3 384  | sha3      | 0.10.6    | https://crates.io/crates/sha3/0.10.6 |
| `SHA3-512`  | SHA-3 512  | sha3      | 0.10.6    | https://crates.io/crates/sha3/0.10.6 |
|`RipeMD-128` | RipeMD-128 | ripemd    | 0.1.3     | https://crates.io/crates/ripemd/0.1.3 |
|`RipeMD-160` | RipeMD-160 | ripemd    | 0.1.3     | https://crates.io/crates/ripemd/0.1.3 |
|`RipeMD-256` | RipeMD-256 | ripemd    | 0.1.3     | https://crates.io/crates/ripemd/0.1.3 |
|`RipeMD-320` | RipeMD-320 | ripemd    | 0.1.3     | https://crates.io/crates/ripemd/0.1.3 |

## Installation
```
git clone https://github.com/DaddyBigFish/fishcrack.git ~/.git/fishcrack
cd ~/.git/fishcrack
cargo build --release
sudo ln -sf $(realpath ~/.git/fishcrack/target/release/fishcrack) /usr/local/bin/fishcrack
chmod +x /usr/local/bin/fishcrack
```
## Usage
#### <hash_type>
```
fishcrack ~/rockyou SHA3-512 48f4892c4227b737630fe6af9e69592f999849d2c5bfb0f02950c3040605245292880bdf169fcc3fb147a808435d6dbbb6d124c2a4c1242602230c00a20816bc
🐠 fishcrack v0.1
Loaded the wordlist file in 383 millisecs.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💛 Cracked!   🔑 securepassword   ⏳ 436 millisecs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```
#### --spray
```
 fishcrack ~/rockyou --spray 48f4892c4227b737630fe6af9e69592f999849d2c5bfb0f02950c3040605245292880bdf169fcc3fb147a808435d6dbbb6d124c2a4c1242602230c00a20816bc
🐠 fishcrack v0.1
Loaded the wordlist file in 378 millisecs.
Now spraying! Attempting all hash algorithms.
Filtered algorithms for length 128: ["MD6-512", "SHA-512", "SHA3-512"]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💛 Cracked!   🔑 securepassword   ⏳ 2061 millisecs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```
