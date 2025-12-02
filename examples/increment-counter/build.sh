#!/usr/bin/env bash

set -euo pipefail

TARGET_JSON_PATH=$(polkatool get-target-json-path)
CARGO_TARGET_DIR="$PWD/target"

# Fix for rustc `target-pointer-width` type breaking change, see
# https://github.com/paritytech/polkadot-sdk/issues/10400
sed -i 's/"target-pointer-width": "64",/"target-pointer-width": 64,/' $TARGET_JSON_PATH

RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
    CARGO_TARGET_DIR=$CARGO_TARGET_DIR \
    cargo +nightly build \
        -Z build-std=core,alloc \
        --target $TARGET_JSON_PATH \
        --release

CRATE=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/')

polkatool link \
    --strip --run-only-if-newer \
    "$CARGO_TARGET_DIR/riscv64emac-unknown-none-polkavm/release/${CRATE}" \
    -o "${CRATE}.polkavm"
