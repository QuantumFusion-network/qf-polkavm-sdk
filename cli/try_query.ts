import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import fs from "fs";
import { generateAddress, keccak256 } from "ethereumjs-util";
import {
    AddressOrPair,
    ApiTypes,
    SubmittableExtrinsic,
} from "@polkadot/api/types";
import { ISubmittableResult } from "@polkadot/types/types";

async function signSendAndWait<
    ApiType extends ApiTypes,
    R extends ISubmittableResult,
>(
    tx: SubmittableExtrinsic<ApiType, R>,
    signer: AddressOrPair,
    cb: (e: R, done: () => void) => void,
) {
    await new Promise((resolve, reject) => {
        let unsub: () => void;
        tx.signAndSend(signer, (e) => {
            cb(e, () => {
                unsub?.();
                resolve(null);
            });
            if (e.status.isInBlock) {
                unsub?.();
                resolve(null);
            }
        })
            .then((u: () => void) => {
                unsub = u;
            })
            .catch(reject);
    });
}

async function mapAccount(api: ApiPromise, alice: KeyringPair) {
    const tx = api.tx.revive.mapAccount();
    const unsub = await tx.signAndSend(alice, ({ status, events }) => {
        if (status.isInBlock) {
            for (const { event } of events) {
                console.log(`Event: ${event.section}.${event.method}`);
            }
            unsub();
        }
    });
    console.log("mapAccount done");
}

async function uploadContract(api: ApiPromise, alice: KeyringPair) {
    const codeBuffer = fs.readFileSync("../output/increment-counter.polkavm");

    const codeHash = keccak256(codeBuffer).toString("hex");

    const storageDepositLimit = null; // or a BN/u128
    const tx = api.tx.revive.uploadCode([...codeBuffer], storageDepositLimit);

    const unsub = await tx.signAndSend(alice, ({ status, events }) => {
        if (status.isInBlock) {
            for (const { event } of events) {
                console.log(`Event: ${event.section}.${event.method}`);
            }
            unsub();
        }
    });
    console.log("uploadCode done");
    return codeHash;
}

async function instantiateContract(
    api: ApiPromise,
    alice: KeyringPair,
    codeHash: string,
) {
    const constructorData = "0x0400";
    const value = "1000000000000000000"; // Amount in native tokens to send to contract
    const gasLimit = api.registry.createType("SpWeightsWeightV2Weight", {
        refTime: "1000000000",
        proofSize: "1000000000",
    });
    const storageDepositLimit = "1000000000000000000";
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

    const unsub = await tx.signAndSend(alice, ({ status, events }) => {
        if (status.isInBlock) {
            for (const { event } of events) {
                console.log(`Event: ${event.section}.${event.method}`);
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
                    console.log("Contract instantiated", event.data.toHuman());
                }
            }
            unsub();
        }
    });
    console.log("instantiate done");
}

async function getNonce(api: ApiPromise, alice: KeyringPair) {
    const accountInfo = await api.query.system.account(alice.address);
    return (accountInfo.toJSON() as any).nonce as number;
}

// this is an (ed|sr)25510 derived address
function getEthAddr(account32: Uint8Array) {
    const h = keccak256(Buffer.from(account32));
    return h.subarray(12); // last 20 bytes
}

// minimal big-endian bytes for a nonnegative integer (0 -> empty, as per RLP)
function bigintToMinimalBytes(n: bigint) {
    if (n < 0n) throw new Error("nonce must be nonnegative");
    if (n === 0n) return new Uint8Array(0);
    let hex = n.toString(16);
    if (hex.length % 2) hex = "0" + hex;
    const out = new Uint8Array(hex.length / 2);
    for (let i = 0; i < out.length; i++)
        out[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
    return out;
}

function create1(ethAddr: Uint8Array, nonce: number) {
    const n = typeof nonce === "number" ? BigInt(nonce) : nonce;
    const nonceBytes = bigintToMinimalBytes(n);
    return generateAddress(Buffer.from(ethAddr), Buffer.from(nonceBytes));
}

async function callContract(
    api: ApiPromise,
    alice: KeyringPair,
    contractAddress: string,
) {
    const value = "1000000000000000000"; // Amount in native tokens to send to contract
    const gasLimit = api.registry.createType("SpWeightsWeightV2Weight", {
        refTime: "1000000000",
        proofSize: "1000000000",
    });
    const storageDepositLimit = "1000000000000000000";
    const data = "0x00";

    // Call the contract
    const tx = api.tx.revive.call(
        `0x${contractAddress}`,
        value,
        gasLimit,
        storageDepositLimit,
        data,
    );

    const unsub = await tx.signAndSend(alice, ({ status, events }) => {
        if (status.isInBlock) {
            for (const { event } of events) {
                console.log(
                    "callContract",
                    `Event: ${event.section}.${event.method}`,
                );
                if (
                    event.section === "system" &&
                    event.method === "ExtrinsicFailed"
                ) {
                    console.log("callContract", event.data.toHuman());
                }
                if (
                    event.section === "revive" &&
                    event.method === "ContractEmitted"
                ) {
                    console.log(
                        "callContract",
                        "Contract called",
                        event.data.toHuman(),
                    );
                }
            }
            unsub();
        }
    });
    console.log("callContract", "called");
}

async function main() {
    const wsProvider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({
        provider: wsProvider,
        noInitWarn: true,
        throwOnConnect: true,
    });

    const keyring = new Keyring({ type: "sr25519" });
    const alice = keyring.addFromUri("//Alice");

    await mapAccount(api, alice);
    const codeHash = await uploadContract(api, alice);

    const nonce = await getNonce(api, alice);
    const ethAddr = getEthAddr(alice.addressRaw);
    const contractAddress = create1(ethAddr, nonce).toString("hex");
    console.log(`contract address 0x${contractAddress}`);

    await instantiateContract(api, alice, codeHash);

    await callContract(api, alice, contractAddress);
}

main().catch(console.error);
