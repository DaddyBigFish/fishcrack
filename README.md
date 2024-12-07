## Supported hashes
```
    let valid_hashes = vec![
    	"MD2",
    	"MD4",
    	"MD5",
    	"SHA-1",
    	"RipeMD-128",
    	"RipeMD-160",
    	"RipeMD-256",
    	"RipeMD-320",
    	"SHA-224",
    	"SHA-256",
    	"SHA-384",
    	"SHA-512",
    	"SHA3-224",
    	"SHA3-256",
    	"SHA3-384",
    	"SHA3-512"
    ];
```

## Installation
```
git clone https://github.com/DaddyBigFish/fishcrack.git ~/.git/fishcrack
cd ~/.git/fishcrack
cargo build --release
sudo ln -sf $(realpath ~/.git/ruo/target/release/ruo) /usr/local/bin/fishcrack
chmod +x /usr/local/bin/fishcrack
```
