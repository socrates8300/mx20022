# Build stage
FROM rust:1.85-bookworm AS builder
RUN cargo install cargo-deny@0.16 && cargo install just@1
WORKDIR /build
COPY . .
RUN just ci

# Runtime stage (CLI binary only)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/mx20022-cli /usr/local/bin/mx
ENTRYPOINT ["mx"]
