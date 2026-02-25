set shell := ["bash", "-euc"]

pre-commit: fmt check test

fmt:
    cargo fmt --all

check:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings

test:
    cargo test --locked