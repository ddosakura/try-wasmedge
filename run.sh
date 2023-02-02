cargo build --target wasm32-wasi
wasmedge --env RUST_BACKTRACE=full target/wasm32-wasi/debug/try-wasm.wasm
# wasmedge target/wasm32-wasi/debug/try-wasm.wasm
