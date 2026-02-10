#!/bin/bash
set -e

echo "Running tests..."
cargo test --workspace

echo "Running clippy..."
cargo clippy --workspace -- -D warnings

echo "Checking formatting..."
cargo fmt -- --check

echo "All checks passed!"
