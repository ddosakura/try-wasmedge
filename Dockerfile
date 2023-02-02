# docker buildx build --platform wasi/wasm32 -t try-wasmedge:<version> .

FROM scratch
ENTRYPOINT [ "bin.wasm" ]
COPY target/bin.wasm /bin.wasm
