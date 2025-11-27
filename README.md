<div align="center">

# QF Network PolkaVM SDK

[![License](https://img.shields.io/github/license/QuantumFusion-network/qf-solochain?color=green)](https://github.com/QuantumFusion-network/qf-polkavm-sdk/blob/main/LICENSE)
<br>
![GitHub contributors](https://img.shields.io/github/contributors/QuantumFusion-network/qf-polkavm-sdk)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/QuantumFusion-network/qf-polkavm-sdk)
![GitHub last commit](https://img.shields.io/github/last-commit/QuantumFusion-network/qf-polkavm-sdk)
<br>
[![Twitter URL](https://img.shields.io/twitter/follow/theqfnetwork?style=social)](https://x.com/theqfnetwork)

</div>

The SDK simplifies the development of native Rust smart contracts on QF Network. Smart contract functionality in QF Network is implemented using [pallet-revive](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/index.html) from Polkadot SDK / Substrate. See [pallet-revive-uapi](https://paritytech.github.io/polkadot-sdk/master/pallet_revive_uapi/index.html) for reference documentation on the API available to smart contract developers.

To contribute to this project, please read the [Contributing](#contributing) section.

## Features

Existing and planed features.

- [x] Simplified smart contract project compilation
- [x] Essentials: allocator, panic handler
- [x] `export` macro which simplifies `call` and `deploy` definitions
- [ ] Improved storage layer API (typed / structs-based, instead of key-value API of the `pallet-revive-uapi`)
- [ ] JavaScript / TypeScript client library generation
- [ ] Examples
- [ ] Deployment and testing tools

## Compile example

To compile a smart contract we need to install a correct version of `polkatool` (it should match chain's `pallet-revive` version) and run a script that configures environment and invokes it with correct arguments.

1. Install `polkatool`

    ```bash
    cargo install --git https://github.com/paritytech/polkavm.git --tag v0.21.0 polkatool
    ```

1. Build a smart contract from `examples/increment-counter`

    ```console
    ./build_polkavm.sh increment-counter
    ```

1. The `*.polkavm` binary for the deployment should be available at the following path

    ```console
    output/increment-counter.polkavm
    ```

## Deploy and call a smart contract

```bash
./build_polkavm.sh increment-counter
```

See [guides.md](guides.md) for explaination on how to deploy smart contract on a [QF Network Portal](https://portal.qfnetwork.xyz/).

To run tests on a [local network](https://github.com/QuantumFusion-network/qf-solochain/blob/main/zombienet/README.md) run:
```bash
cd cli
npx ts-node upload_and_execute.ts ws://127.0.0.1:9944 ../output/increment-counter.polkavm
```

## Debugging

Run the node with `pallet-revive` logs and historical state.

```console
qf-node --dev -lerror,runtime::revive::strace=trace,runtime::revive=debug --state-pruning archive
```

See also <https://github.com/paritytech/polkadot-sdk/blob/598feddb/substrate/frame/revive/README.md#host-function-tracing>.

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
