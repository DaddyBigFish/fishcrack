## Introduction
![image](https://github.com/user-attachments/assets/1c549bbc-a15c-478c-8c12-c44d1fc48b1d)
Rust hash cracker inspired by the project Ruo.

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
fishcrack ~/rockyou MD5 f25a2fc72690b780b2a14e140ef6a9e0
🐠 fishcrack v0.1
Loaded MD5 hash.
loaded the wordlist file in 342 millisecs.
🤍 Cracked! f25a2fc72690b780b2a14e140ef6a9e0 -> "iloveyou" in 0 millisecs

fishcrack ~/rockyou SHA3-512 e9a75486736a550af4fea861e2378305c4a555a05094dee1dca2f68afea49cc3a50e8de6ea131ea521311f4d6fb054a146e8282f8e35ff2e6368c1a62e909716
🐠 fishcrack v0.1
Loaded SHA3-512 hash.
loaded the wordlist file in 335 millisecs.
🤍 Cracked! e9a75486736a550af4fea861e2378305c4a555a05094dee1dca2f68afea49cc3a50e8de6ea131ea521311f4d6fb054a146e8282f8e35ff2e6368c1a62e909716 -> "password" in 0 millisecs
```
#### --spray
```
fishcrack ~/rockyou --spray e21a255ec884c1287320227b0c915ed8c34ee45ba5828d0fa85d6b8984aace8d7b12e5904af84a457be8bfa0941eef84f40b03ad12b4edc7f0bfc092b59b5419
🐠 fishcrack v0.1
Loaded the wordlist file in 322 millisecs.
Now spraying! Attempting all hash algorithms.
🤍 Cracked! e21a255ec884c1287320227b0c915ed8c34ee45ba5828d0fa85d6b8984aace8d7b12e5904af84a457be8bfa0941eef84f40b03ad12b4edc7f0bfc092b59b5419 -> "donkey" in 4490 millisecs
```
