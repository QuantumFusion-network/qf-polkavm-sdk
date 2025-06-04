#!/usr/bin/env bash

set -euo pipefail

echo "Building documentation for main library..."
cargo doc --no-deps

echo "Building documentation for examples..."
for example_dir in examples/*/; do
    if [ -f "${example_dir}/Cargo.toml" ]; then
        example_name=$(basename "$example_dir")
        echo "Building docs for ${example_name}..."

        pushd "$example_dir" > /dev/null

        RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
             cargo +nightly doc \
                --no-deps -Z build-std=core,alloc \
                --target $(polkatool get-target-json-path --bitness 32)

        popd > /dev/null
    fi
done
