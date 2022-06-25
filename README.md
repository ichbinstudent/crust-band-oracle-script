# Band Protocol Oracle Script for Crust File Metadata

Commands to build the script:
```bash
rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
```
