# syntax=docker/dockerfile:1
# docker buildx build -f quickjs.Dockerfile -t try-wasmedge .

FROM ddosakura/hakke:1.2.0-lit AS base
# https://github.com/second-state/wasmedge-quickjs/releases
USER root
RUN wget https://github.com/second-state/wasmedge-quickjs/releases/download/v0.5.0-alpha/wasmedge_quickjs.wasm
RUN git clone https://github.com/second-state/wasmedge-quickjs.git

FROM wasmedge/slim-runtime:0.11.2
COPY --link --from=base /wasmedge_quickjs.wasm /wasmedge_quickjs.wasm
COPY --link --from=base /wasmedge-quickjs/modules /modules
ADD src/main.js /
CMD ["wasmedge", "--dir", ".:/", "/wasmedge_quickjs.wasm", "main.js"]
