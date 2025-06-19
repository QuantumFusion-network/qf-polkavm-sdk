#!/usr/bin/env bash

set -euo pipefail

echo "Building documentation for all workspace members..."

RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
    cargo +nightly doc \
        --workspace \
        --no-deps \
        -Z build-std=core,alloc \
        --target $(polkatool get-target-json-path --bitness 32)

echo "Documentation build complete!"
