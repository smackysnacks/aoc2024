set positional-arguments

alias r := run
alias t := test
alias tl := test-libs
alias c := check
alias cl := check-libs
alias b := build
alias l := lint

help:
    @just --list

# Run the cli
run:
    cargo run

# Run cargo check on workspace
check:
    cargo check --workspace

# Run cargo check on workspace excluding main binary crate
check-libs:
    cargo check --workspace --exclude aoc2024

# Run cargo build on workspace
build:
    cargo build --workspace

# Run cargo nextest on workspace
test *args:
    #!/usr/bin/env bash
    if ! command -v cargo-nextest >/dev/null; then
        echo "cargo-nextest not found. You can install it by running: cargo install cargo-nextest"
        exit 1
    fi
    cargo nextest run --no-tests=warn --workspace "$@"

# Run cargo nextest on workspace excluding main binary crate
test-libs *args:
    #!/usr/bin/env bash
    if ! command -v cargo-nextest >/dev/null; then
        echo "cargo-nextest not found. You can install it by running: cargo install cargo-nextest"
        exit 1
    fi
    cargo nextest run --no-tests=warn --workspace --exclude aoc2024 "$@"

# Test and produce a code coverage report
coverage:
    #!/usr/bin/env bash
    if ! command -v cargo-llvm-cov &>/dev/null || ! command -v cargo-nextest &>/dev/null; then
        echo "cargo-nextest or cargo-llvm-cov not found. You can install them by running: cargo install cargo-llvm-cov cargo-nextest"
        exit 1
    fi
    cargo llvm-cov nextest --workspace

# Run cargo clippy on workspace
lint:
    cargo clippy --workspace

# Scan Cargo.lock for known vulnerabilities in dependencies
audit:
    #!/usr/bin/env bash
    if ! command -v cargo-audit >/dev/null; then
        echo "cargo-audit not found. You can install it by running: cargo install cargo-audit"
        exit 1
    fi
    cargo audit

# Show outdated dependencies
show-outdated:
    cargo outdated --workspace
