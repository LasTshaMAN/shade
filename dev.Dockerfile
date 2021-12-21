#FROM enigmampc/secret-network-sw-dev
FROM rust:latest

RUN apt-get update && apt-get -y install \
    build-essential \
    clang \
    binaryen

## Get Rust, note: using sh for better compatibility with other base images.
#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
## Add .cargo/bin to PATH.
#ENV PATH="/root/.cargo/bin:${PATH}"

#RUN rustup default stable && \
#    rustup target list --installed && \
#    rustup target add wasm32-unknown-unknown && \
#    rustup install nightly && \
#    rustup target add wasm32-unknown-unknown --toolchain nightly && \
#    cargo install cargo-generate --features vendored-openssl

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    cargo install cargo-generate --features vendored-openssl

WORKDIR /home
