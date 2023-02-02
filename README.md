# docs

- https://github.com/second-state/wasmedge_wasi_socket
- https://github.com/second-state/http_req
- https://github.com/WasmEdge/tokio
- https://github.com/WasmEdge/hyper

- https://webassembly.github.io/wabt/demo/wasm2wat/
- https://httpbin.org

```rust
// thread 'main' panicked at 'OS can't spawn worker thread: operation not supported on this platform'

// fix by:
// https://github.com/WasmEdge/WasmEdge/issues/1429
// https://github.com/WasmEdge/WasmEdge/issues/2136
#[tokio::main(flavor = "current_thread")]
async fn main() {}
```
