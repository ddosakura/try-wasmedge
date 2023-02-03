# docker build -f slim.Dockerfile -t try-wasmedge .

# https://hub.docker.com/r/wasmedge/slim-runtime/tags
# **没有插件，无法运行 [WasmEdge-HttpsReq Plug-in](https://wasmedge.org/book/en/write_wasm/rust/networking-https.html#prerequisites)**
FROM wasmedge/slim-runtime:0.11.2
COPY target/bin.wasm /bin.wasm
CMD ["wasmedge", "--dir", ".:/", "/bin.wasm"]
