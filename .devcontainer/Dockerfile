FROM rust:1.85-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev sqlite3 \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy llvm-tools-preview \
    && rustup toolchain install 1.75.0 --no-self-update

RUN cargo install cargo-deny@0.16 \
    && cargo install cargo-insta@1 \
    && cargo install just@1 \
    && cargo install cargo-llvm-cov@0.6

RUN rm -rf /usr/local/cargo/registry/src

ENV CARGO_TERM_COLOR=always
