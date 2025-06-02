<div align="center">

# QF Network PolkaVM SDK

[![License](https://img.shields.io/github/license/QuantumFusion-network/qf-solochain?color=green)](https://github.com/QuantumFusion-network/qf-polkavm-sdk/blob/main/LICENSE)
<br>
![GitHub contributors](https://img.shields.io/github/contributors/QuantumFusion-network/qf-polkavm-sdk)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/QuantumFusion-network/qf-polkavm-sdk)
![GitHub last commit](https://img.shields.io/github/last-commit/QuantumFusion-network/qf-polkavm-sdk)
<br>
[![Twitter URL](https://img.shields.io/twitter/follow/QuantumFusion_?style=social)](https://x.com/QuantumFusion_)

</div>

This framework enables the development of smart contracts for the Quantum Fusion Network. For the smart contract platform implementation details, please refer to the [PolkaVM pallet documentation](https://github.com/QuantumFusion-network/spec/blob/main/docs/PolkaVM/polkavm_pallet.md).

For contributing to this project, please read [Contributing](#contributing) section.

## Compiling example Smart Contract

The QF Network executes smart contracts in the PolkaVM virtual machine and requires PolkaVM tools for smart contracts compilation.

1. Install `polkatool`.

    ```bash
    cargo install --git https://github.com/paritytech/polkavm.git --tag v0.21.0 polkatool
    ```

1. Build smart-contract `examples/hello-qf-polkavm`.

    ```bash
    ./build_polkavm.sh hello-qf-polkavm
    ```

    Or compile manually:
    ```bash
    export CRATE_NAME=hello-qf-polkavm
    mkdir -p output

    pushd "examples/${CRATE_NAME}"
    RUSTFLAGS="--remap-path-prefix=$(pwd)= --remap-path-prefix=${HOME}=~" \
        cargo +nightly build \
            -Z build-std=core,alloc \
            --target $(polkatool get-target-json-path --bitness 32) \
            -q --release --bin "${CRATE_NAME}" -p "${CRATE_NAME}"
    popd

    polkatool link \
        --run-only-if-newer \
        -s "examples/${CRATE_NAME}/target/riscv32emac-unknown-none-polkavm/release/${CRATE_NAME}" \
        -o "output/${CRATE_NAME}.polkavm"
    ```

## Deploy and call a smart contract

```bash
./build_polkavm.sh increment-counter
```

See [guides.md](guides.md) for explaination on how to deploy smart contract on a [QF Network Portal](https://portal.qfnetwork.xyz/).

To run tests on a [local network](https://github.com/QuantumFusion-network/qf-solochain/blob/main/zombienet/README.md) run:
```bash
cd cli
npx ts-node uploadAndExecute.ts wss://localhost:<LOCAL_PORT> ../output/increment-counter.polkavm
```

## Contributing

We welcome contributions of all kinds! Whether you're reporting or fixing a bug, adding a feature, or improving
documentation, your help is greatly appreciated. For a bug or vulnerability report please [open a new issue](https://github.com/QuantumFusion-network/qf-polkavm-sdk/issues).

For code contributions please follow these steps:

1. Fork the repository and create a new branch following the format `your-github-name/descriptive-branch-name` (e.g., `qf-polkavm-sdk/fix-123`).
2. Make smaller commits with clear messages to simplify reviewer's work.
3. Submit a pull request targeting `main` branch and provide a concise description of your changes.

By contributing, you agree to adhere to our [Contributor Covenant Code of Conduct](./CODE_OF_CONDUCT.md), which fosters
a respectful and inclusive environment.

We appreciate your support and look forward to your contributions! 🚀
