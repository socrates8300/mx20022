default: check test clippy fmt-check

check:
    cargo check --workspace --all-features

test:
    cargo test --workspace --all-features

clippy:
    cargo clippy --workspace --all-features -- -D warnings

fmt-check:
    cargo fmt --all -- --check

fmt:
    cargo fmt --all

deny:
    cargo deny check

ci: check test clippy fmt-check deny doc

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo doc --workspace --all-features --no-deps

coverage:
    cargo llvm-cov --workspace --all-features --html
    @echo "Report: target/llvm-cov/html/index.html"

bench:
    cargo bench --workspace

codegen *ARGS:
    cargo run --package mx20022-codegen -- {{ARGS}}

review:
    cargo insta review

msrv:
    cargo +1.75.0 check --workspace --all-features
