cargo build --target wasm32-wasi --release
wasmedgec target/wasm32-wasi/release/try-wasmedge.wasm target/bin.wasm
# wasmedge target/bin.wasm
