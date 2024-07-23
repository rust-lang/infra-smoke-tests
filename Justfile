# List available commands
_default:
    just --list

format *ARGS:
    cargo fmt --all -- {{ARGS}}

lint:
    cargo clippy --all-targets --all-features -- -D warnings

run *ARGS:
    cargo run -- {{ARGS}}

test:
    cargo test --all-features --all-targets
