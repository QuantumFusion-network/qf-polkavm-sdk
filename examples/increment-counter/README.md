# Increment counter

A minimal example smart contract.

## Prerequisites

This project requires `polkatool` to execute the linking step after compilation.

```console
cargo install --git https://github.com/paritytech/polkavm.git --tag v0.21.0 polkatool
```

## Usage

1. Create a copy of this project.

    ```console
    git clone --depth=1 https://github.com/QuantumFusion-network/qf-polkavm-sdk.git
    cp -r qf-polkavm-sdk/examples/increment-counter .
    cd increment-counter
    ```

1. Read `src/main.rs` to understand what it does.

1. Create contract's binary for deployment using the build script.

    ```console
    ./build.sh
    ```

    This creates a binary file named `increment-counter.polkavm` in the root of the project, ready for deployment.

1. Upload the script at `portal.qfnetwork.xyz` by calling the `revive::instantiateWithCode` extrinsic.

    > Use <https://faucet.qfnetwork.xyz/> to get testnet tokens.

1. Open `call.js` to see how to call the smart contract.

## What's next?

1. Find more examples in Polkadot SDK at <https://github.com/paritytech/polkadot-sdk/blob/4c74a10966/substrate/frame/revive/fixtures/contracts>. You can discover how to read the caller's address, verify transaction signatures, transfer balance, and much more.

1. Tweak the project sources to explore how it works.

1. Share your thoughts by [creating an issue](https://github.com/QuantumFusion-network/qf-polkavm-sdk/issues/new) in the SDK project repository if you have ideas on how to improve native Rust smart contract development by adding or changing something in the SDK!
