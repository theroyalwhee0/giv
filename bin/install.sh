#!/usr/bin/env bash

# Run Clippy.
cargo clippy --all-targets --all-features -- -D warnings || exit 1

# Run tests.
cargo test || exit 1

# Install the binary.
cargo install --path . || exit 1
