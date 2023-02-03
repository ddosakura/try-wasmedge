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

- [docker 添加 crun 支持](https://github.com/containers/crun/issues/45)
- [crun 的 wasmedge 支持](https://wasmedge.org/book/en/use_cases/kubernetes/container/crun.html)
- [wasmedge 版 crun 无法静态构建](https://github.com/containers/crun/issues/922)
- [基于 wasmedge/slim-runtime 镜像运行](https://wasmedge.org/book/en/use_cases/kubernetes/docker/lxc.html)
  - https://github.com/second-state/wasmedge-quickjs
