import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { u64, Option, u32, i64 } from "@polkadot/types";
import { Codec } from "@polkadot/types/types";
import fs from "fs";

function buf2hex(buffer: Uint8Array) {
    return [...new Uint8Array(buffer)]
        .map((x) => x.toString(16).padStart(2, "0"))
        .join("");
}

async function main() {
    // Retrieve the WS endpoint from command-line arguments
    const wsEndpoint = process.argv[2] || "wss://test.qfnetwork.xyz";
    const programPath = process.argv[3];

    if (!fs.existsSync(programPath)) {
        console.error(`Program file not found: ${programPath}`);
        process.exit(1);
    }

    // Connect to the node
    const wsProvider = new WsProvider(wsEndpoint);
    const api = await ApiPromise.create({
        provider: wsProvider,
        noInitWarn: true,
    });

    // Initialize keyring and add Alice account
    const keyring = new Keyring({ type: "sr25519" });
    const ALICE = keyring.addFromUri("//Alice");

    // Prepare program data
    const fileBuffer = fs.readFileSync(programPath);
    const programData = new Uint8Array(fileBuffer);

    console.log(`Prepared program blob of size ${programData.length} bytes`);

    // Upload the program
    const uploadExtrinsic = api.tx.qfPolkaVM.upload(
        "0x" + buf2hex(programData),
    );
    console.log(
        `Extrinsic created: ${uploadExtrinsic.method.section}.${uploadExtrinsic.method.method}`,
    );

    const contractAddress: string = await new Promise((resolve, reject) => {
        uploadExtrinsic
            .signAndSend(ALICE, ({ events = [], status }) => {
                events.forEach(({ event: { data, method, section } }) => {
                    if (
                        section === "qfPolkaVM" &&
                        method === "ProgramBlobUploaded"
                    ) {
                        const address = data[1].toString();
                        resolve(address);
                    }
                });
            })
            .catch(reject);
    });

    console.log("Program uploaded successfully!");
    console.log(`Contract address: ${contractAddress}`);

    // Fund the smart contract account
    const contractFundAmount = "1000000000000000000";
    const transfer = api.tx.balances.transferAllowDeath(
        contractAddress,
        contractFundAmount,
    );

    await new Promise<void>((resolve, reject) => {
        transfer
            .signAndSend(ALICE, ({ events = [], status }) => {
                events.forEach(({ event: { data, method, section } }) => {
                    if (section === "balances" && method === "Transfer") {
                        resolve();
                    }
                });
            })
            .catch(reject);
    });

    console.log("Contract funded successfully!");

    // Execute the contract
    const to = ALICE.address;
    const value = 10;

    // Data to pass to smart contract
    const n = 42;
    const u32Value = api.createType("u32", n);
    const userDataRaw = u32Value.toU8a();
    const userData = "0x" + Buffer.from(userDataRaw).toString("hex");

    const gasLimit = 2000000;
    const gasPrice = 10;

    const execute = async () => {
        const executeExtrinsic = api.tx.qfPolkaVM.execute(
            contractAddress,
            to,
            value,
            userData,
            gasLimit,
            gasPrice,
        );
        console.log(
            `Extrinsic created: ${executeExtrinsic.method.section}.${executeExtrinsic.method.method}`,
        );

        const executionData: Codec[] = await new Promise((resolve, reject) => {
            executeExtrinsic
                .signAndSend(ALICE, ({ events = [], status }) => {
                    events.forEach(({ event: { data, method, section } }) => {
                        if (
                            section === "qfPolkaVM" &&
                            method === "ExecutionResult"
                        ) {
                            resolve(data);
                        }
                    });
                })
                .catch(reject);
        });

        const [
            _who,
            _contractAddress,
            version,
            result,
            notEnoughGas,
            trap,
            gasBefore,
            gasAfter,
        ] = executionData;

        console.log("Program executed successfully!");
        console.log(
            JSON.stringify(
                {
                    version: (version as u64).toHuman(),
                    result: (result as Option<u64>).toHuman(),
                    notEnoughGas,
                    trap,
                    gasBefore: (gasBefore as u32).toHuman(),
                    gasAfter: (gasAfter as i64).toHuman(),
                },
                null,
                2,
            ),
        );
    };

    // First execute
    await execute();

    // Second execute
    await execute();

    // Disconnect from the node
    await api.disconnect();
}

main().catch(console.error);
