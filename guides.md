# Guides

## Compile your first smart contract

See [Compiling example Smart Contract](README.md#compiling-example-smart-contract).

## Deploy the compiled smart contract

1. Open the QF Network Portal [^1] at <https://portal.qfnetwork.xyz/> and connect your wallet accounts.

1. Click the logo in the top-left corner to open network selector.

1. Expand `DEV NETWORKS` group and select `QF Devnet` (at the time of writing, smart contracts are only available on QF Devnet).

1. Hover over `Developer` in the top menu and click on `Extrinsics`.

1. Select the account you want to use for deployment.

1. Below in the runtime module dropdown (default is `system`), select `qfPolkaVM`.

1. To the right of the runtime module dropdown, in the extrinsic selector (default is `execute(contractAddress, ...)`), select `upload(programBlob)`.

1. Below in the `programBlob: Bytes` input field, toggle `file upload` on the right.

1. Click or drag-and-drop the compiled smart contract output into the upload area (`click to select or drag and drop the file here`).

1. Click `Submit Transaction`, then `Sign and Submit`, and confirm in your wallet. Your smart contract is now deployed! 🎉

1. To retrieve the deployed contract address navigate to `Developer, Chain state`, select `qfPolkaVM` at the runtime module selector, and `codeAddress((AccountId32,u64)): Option<AccountId32>` to the right of the runtime module selector in the storage item selector.

1. In the account selector, choose the same account used for deployment.

1. In the `u64` field, enter the deployment index (starting from 1) to get the address.

1. To the right-hand of the storage item selector, click `+` to execute the query.

1. Scroll to the bottom to find `qfPolkaVM.codeAddress: Option<AccountId32>`. The address below is your deployed contract address. Use it to interact with the contract. You can use it further it to interact with the contract.

## Call the smart contract function

The following steps assume that you are using the [hello-qf-polkavm](examples/hello-qf-polkavm) smart contract.

1. Open the QF Network Portal [^1] at <https://portal.qfnetwork.xyz/> and connect your wallet accounts.

1. Let's first check the current state of the smart contract before modifying it. Navigate to `Developer, Chain State`.

1. Select the `qfPolkaVM` runtime module and the `codeStorage((AccountId32,Bytes)): Option<Bytes>` storage item.

1. Enter the deployed smart contract address in the first input field under `AccountId32`.

1. Enter the deployment index of the smart contract in the second input field under `u64`. This is the sequential number of the deployment from the selected account (starting from 1).

1. Enter the following string in the third field under `Bytes`. This value is part of the [hello-qf-polkavm](examples/hello-qf-polkavm) contract and represents a hex-encoded key used for accessing a value in the contract's storage.

    ```console
    0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F
    ```

1. Click the `+` button to the right of the runtime module and storage item selectors to send the query.

1. Scroll to the bottom and locate the field under `qfPolkaVM.codeStorage: Option<Bytes>`. This field shows the current stored value which we will change further calling the smart contract function.

    If it shows `<none>`, the contract has not been called yet and the value is unset - a number will appear after the first call.

1. Now let's send a transaction that calls a function of the smart contract. Navigate to `Developer, Extrinsics`.

1. Select the account you want to use for this transaction.

1. Choose the `qfPolkaVM` runtime module and select the `execute(contractAddress, to, value, userData, gasLimit, gasPrice)` extrinsic.

1. Set the following parameters:
    - `contractAddress: AccountId32`: the deployed contract address.
    - `to: AccountId32`: not used for this call, leave the default value.
    - `value: u128`: not used for this call, leave the default value.
    - `userData: Bytes`: set to `0x05`. This value corresponds to the `increment` operation in the [hello-qf-polkavm](examples/hello-qf-polkavm) contract.
    - `gasLimit: u32`: set to `100000`.
    - `gasPrice: u64`: set to `1`. The smart contracts pricing model is under development. Currently, users can set the gas price manually, which enables flexible experimentation.

1. Click `Submit Transaction`, then `Sign and Submit`, and confirm the transaction in your wallet.

1. Repeat steps 2 - 8 to verify the change in stored value. Each call with `0x05 (increment operation)` increments the stored value by 1.

[^1]: [QF Network Portal](https://portal.qfnetwork.xyz/) is a fork of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/).
