#!/bin/bash

# Check if a crate name was provided
if [ $# -eq 0 ]; then
    echo "Error: No crate name provided."
    echo "Usage: $0 <crate_name>"
    echo "Crates: hello-qf-polkavm get-account-balance get-block-number get-contract-balance increment-counter"
    exit 1
fi

# Set the crate name from the first argument
CRATE_NAME=$1

# Create output directory if it doesn't exist
mkdir -p output

# Build and link the crate
pushd "examples/${CRATE_NAME}" > /dev/null

echo "Building ${CRATE_NAME}"

RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
    cargo +nightly build \
        -Z build-std=core,alloc \
        --target $(polkatool get-target-json-path --bitness 32) \
        -q --release --bin "${CRATE_NAME}" -p "${CRATE_NAME}"
popd > /dev/null

polkatool link \
    --run-only-if-newer \
    -s "examples/${CRATE_NAME}/target/riscv32emac-unknown-none-polkavm/release/${CRATE_NAME}" \
    -o "output/${CRATE_NAME}.polkavm"

echo "Successfully built and linked ${CRATE_NAME} to output/${CRATE_NAME}.polkavm"
