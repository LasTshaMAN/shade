# TODO
# Integration tests are currently broken because they require sercretcli
# - either install it in this image
# - or use enigma image from previous commit in this branch
FROM rust:latest

RUN apt-get update && apt-get -y install \
    build-essential \
    clang \
    binaryen

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    cargo install cargo-generate --features vendored-openssl

WORKDIR /home
