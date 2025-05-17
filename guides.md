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

[^1]: [QF Network Portal](https://portal.qfnetwork.xyz/) is a fork of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/).
