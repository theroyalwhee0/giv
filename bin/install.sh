#!/usr/bin/env bash

set -e

# Run Clippy.
cargo clippy --all-targets --all-features -- -D warnings || exit 1

# Run tests.
cargo test || exit 1

# Install the binary.
cargo install --path .

# # Generate the SHA256 checksum for the binary.
# (
#     cd target/release
#     sha256sum giv > giv.sha256
# )
# install_path=$(dirname $(which giv))
# cp target/release/giv.sha256 "$install_path/"
