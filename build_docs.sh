#!/usr/bin/env bash

set -euo pipefail

echo "Building documentation for all workspace members..."

TARGET_JSON_PATH=$(polkatool get-target-json-path)

# Fix for rustc `target-pointer-width` type breaking change, see
# https://github.com/paritytech/polkadot-sdk/issues/10400
sed -i 's/"target-pointer-width": "64",/"target-pointer-width": 64,/' $TARGET_JSON_PATH

RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
    cargo +nightly doc \
        --workspace \
        --no-deps \
        -Z build-std=core,alloc \
        --target $TARGET_JSON_PATH

echo "Documentation build complete!"
