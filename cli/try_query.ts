import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { u64, Option, u32, i64 } from "@polkadot/types";
import { Codec } from "@polkadot/types/types";
import fs from "fs";
import createKeccakHash from "keccak";
import { generateAddress, keccak256 } from "ethereumjs-util";

async function mapAccount(api: ApiPromise, alice: KeyringPair) {
    const tx = api.tx.revive.mapAccount();
    let done = new Promise(async (resolve) => {
        const unsub = await tx.signAndSend(alice, ({ status, events }) => {
            if (status.isInBlock) {
                for (const { event } of events) {
                    console.log("Event:", event.section, event.method);
                }
                unsub();
                resolve(null);
            }
        });
    });
    await done;
    console.log("mapAccount done");
}

async function uploadContract(api: ApiPromise, alice: KeyringPair) {
    const codeBuffer = fs.readFileSync("../output/increment-counter.polkavm");

    const codeHash = createKeccakHash("keccak256")
        .update(codeBuffer)
        .digest("hex");

    const storageDepositLimit = null; // or a BN/u128
    const tx = api.tx.revive.uploadCode([...codeBuffer], storageDepositLimit);

    let done = new Promise(async (resolve) => {
        const unsub = await tx.signAndSend(alice, ({ status, events }) => {
            // console.log("Status:", status);
            if (status.isInBlock) {
                for (const { event } of events) {
                    console.log("Event:", event.section, event.method);
                }
                unsub();
                resolve(null);
            }
        });
    });
    await done;
    console.log("uploadCode done");
    return codeHash;
}

async function instantiateContract(
    api: ApiPromise,
    alice: KeyringPair,
    codeHash: string,
) {
    // Prepare wasm constructor data (e.g., using api-contract Metadata, or ABI)
    const constructorData = "0x0400";
    const value = "1000000000000000000"; // Amount in native tokens to send to contract
    const gasLimit = api.registry.createType("SpWeightsWeightV2Weight", {
        refTime: "1000000000", // adjust as needed
        proofSize: "1000000000",
    });
    const storageDepositLimit = "1000000000000000000"; // or specify value
    const salt = null; // or Uint8Array

    // Instantiate the contract
    const tx = api.tx.revive.instantiate(
        value,
        gasLimit,
        storageDepositLimit,
        `0x${codeHash}`,
        constructorData,
        salt,
    );

    let done = new Promise(async (resolve) => {
        const unsub = await tx.signAndSend(alice, ({ status, events }) => {
            if (status.isInBlock) {
                for (const { event } of events) {
                    console.log("Event:", event.section, event.method);
                    if (
                        event.section === "system" &&
                        event.method === "ExtrinsicFailed"
                    ) {
                        console.log(event.data.toHuman());
                    }
                    if (
                        event.section === "revive" &&
                        event.method === "ContractEmitted"
                    ) {
                        console.log(
                            "Contract instantiated",
                            event.data.toHuman(),
                        );
                    }
                }
                unsub();
                resolve(null);
            }
        });
    });
    await done;
    console.log("instantiate done");
}

async function getNonce(api: ApiPromise, alice: KeyringPair) {
    const accountInfo = await api.query.system.account(alice.address);
    return (accountInfo.toJSON() as any).nonce as number;
}

// this is an (ed|sr)25510 derived address
async function getEthAddr(account32: Uint8Array) {
    const h = keccak256(Buffer.from(account32));
    return h.slice(12); // last 20 bytes
}

async function main() {
    const wsProvider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({
        provider: wsProvider,
        noInitWarn: true,
    });

    const keyring = new Keyring({ type: "sr25519" });
    const alice = keyring.addFromUri("//Alice");

    await mapAccount(api, alice);
    const codeHash = await uploadContract(api, alice);

    const nonce = await getNonce(api, alice);
    const ethAddr = getEthAddr(alice.addressRaw);
    const contractAddress = create1(ethAddr, nonce); // TODO

    await instantiateContract(api, alice, codeHash);
}

async function get_entries() {
    const wsProvider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({
        provider: wsProvider,
        noInitWarn: true,
    });

    await api.disconnect();

    const entries = await api.query.revive.pristineCode.entries();
    console.log("result", entries);

    for (const [storageKey, maybeInfo] of entries) {
        // storageKey.args is the decoded key args for the map (here one H256)
        const [codeHash] = storageKey.args;
        console.log("codeHash:", codeHash.toHex());

        console.log("info:", maybeInfo.toHuman()); // or .toJSON(), .toString()
    }

    // Object.entries(api.query.revive).forEach(([k, v]) => {
    //     try {
    //         v()
    //             .then((r) => {
    //                 console.log(k, r);
    //             })
    //             .catch((e) => console.error("err", k));
    //     } catch {
    //         console.error("err", k);
    //     }
    // });
    /*
  {
    palletVersion: [Getter],
    pristineCode: [Getter],
    codeInfoOf: [Getter],
    contractInfoOf: [Getter],
    immutableDataOf: [Getter],
    deletionQueue: [Getter],
    deletionQueueCounter: [Getter],
    originalAccount: [Getter]
  }
  */
}

main().catch(console.error);

/*
Event: balances Withdraw
Event: transactionPayment TransactionFeePaid
Event: system ExtrinsicSuccess
uploadCode done
Event: balances Withdraw
Event: system NewAccount
Event: balances Endowed
Event: balances Transfer
Event: balances Transfer
Event: revive ContractEmitted
Event: balances Deposit
Event: transactionPayment TransactionFeePaid
Event: system ExtrinsicSuccess
instantiate done
*/
