#!/bin/bash
set -e

echo "Building project..."
cargo build

echo "Running CLI system command..."
cargo run -q -p sysaudit-cli -- system

echo "Running CLI software command (head)..."
cargo run -q -p sysaudit-cli -- software | head -n 20

echo "Running CLI updates command (head)..."
cargo run -q -p sysaudit-cli -- updates | head -n 20

echo "CLI tests completed!"
