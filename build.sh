#!/bin/bash
set -e

echo "Building program..."
cargo build-sbf --manifest-path router/Cargo.toml

echo "Building client..."
cargo build --release --manifest-path client/Cargo.toml

echo "Done! Program at: target/deploy/program.so"
