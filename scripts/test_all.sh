#!/bin/sh
set -e

echo "Running tests..."
cargo test --workspace --all-features

echo "Running clippy..."
cargo clippy --workspace --all-targets -- -D warnings

echo "Checking formatting..."
cargo fmt -- --check

echo "All checks passed!"
