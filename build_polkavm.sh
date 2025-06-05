#!/usr/bin/env bash

set -euo pipefail

# Show help if requested
if [ $# -eq 1 ] && [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Usage: $0 [crate_name]"
    echo ""
    echo "Build PolkaVM examples:"
    echo "  $0                 - Build all examples"
    echo "  $0 <crate_name>    - Build single example"
    echo "  $0 -h, --help      - Show this help"
    echo ""
    echo "Available examples:"
    find examples -maxdepth 1 -type d ! -path examples | xargs basename -a | sed 's/^/  /'
    exit 0
fi

# Check if building all examples or a single one
if [ $# -eq 0 ]; then
    # Get all example crate names
    EXAMPLE_CRATES=($(find examples -maxdepth 1 -type d ! -path examples | xargs basename -a))
    echo "Building ${#EXAMPLE_CRATES[@]} examples: ${EXAMPLE_CRATES[*]}"
else
    EXAMPLE_CRATES=("$1")
    echo "Building example: $1"
fi

# Create output directory if it doesn't exist
mkdir -p output

PACKAGE_FLAGS=""
for crate in "${EXAMPLE_CRATES[@]}"; do
    PACKAGE_FLAGS="${PACKAGE_FLAGS} -p ${crate}"
done

RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
    cargo +nightly build \
        -Z build-std=core,alloc \
        --target $(polkatool get-target-json-path --bitness 32) \
        --release \
        ${PACKAGE_FLAGS}

# Link all built examples
for crate in "${EXAMPLE_CRATES[@]}"; do
    echo "Linking ${crate}..."
    polkatool link \
        --run-only-if-newer \
        -s "target/riscv32emac-unknown-none-polkavm/release/${crate}" \
        -o "output/${crate}.polkavm"
done

if [ ${#EXAMPLE_CRATES[@]} -eq 1 ]; then
    echo "Successfully built and linked ${EXAMPLE_CRATES[0]} to output/${EXAMPLE_CRATES[0]}.polkavm"
else
    echo "Successfully built and linked ${#EXAMPLE_CRATES[@]} examples to output/"
fi
