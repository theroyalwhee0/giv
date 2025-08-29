#!/usr/bin/env bash

set -e

# Run tests.
cargo test || exit 1

# Install the binary.
cargo install --path .

# Generate the SHA256 checksum for the binary.
(
    cd target/release
    sha256sum giv > giv.sha256
)
install_path=$(dirname $(which giv))
cp target/release/giv.sha256 "$install_path/"
