import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { u64, Option, u32, i64 } from "@polkadot/types";
import { Codec } from "@polkadot/types/types";
import fs from "fs";
import * as path from "path";
import { spawn, ChildProcess } from "child_process";
import * as readline from "readline";

interface TailOptions {
    lines?: number; // Number of initial lines to show (default: 10)
    encoding?: BufferEncoding; // File encoding (default: 'utf8')
}

/**
 * Tail a file with optional grep-like filtering, similar to `tail -f | grep`
 * @param filePath - Path to the file to tail
 * @param filterPattern - Optional regex pattern or string to filter lines
 * @param options - Additional options for tailing
 * @returns Promise that resolves when the file watcher is set up
 */
export async function tailWithGrep(
    filePath: string,
    filterPattern?: string | RegExp,
    options: TailOptions = {},
): Promise<() => void> {
    const { lines = 10, encoding = "utf8" } = options;

    // Validate file exists
    if (!fs.existsSync(filePath)) {
        throw new Error(`File does not exist: ${filePath}`);
    }

    // Convert string pattern to RegExp if needed
    const regex = filterPattern
        ? typeof filterPattern === "string"
            ? new RegExp(filterPattern, "i") // Case insensitive by default
            : filterPattern
        : null;

    // Function to check if line matches filter
    const matchesFilter = (line: string): boolean => {
        if (!regex) return true;
        return regex.test(line);
    };

    // Function to print filtered line
    const printLine = (line: string): void => {
        if (matchesFilter(line)) {
            // Cut out content before 'polkavm'
            const polkavmIndex = line.indexOf("polkavm");
            const outputLine =
                polkavmIndex !== -1 ? line.substring(polkavmIndex) : line;
            // Force immediate output without buffering
            process.stdout.write("> " + outputLine + "\n");
        }
    };

    // Read initial lines (like tail does)
    const stats = fs.statSync(filePath);
    let fileSize = stats.size;

    // Read last N lines initially
    const initialContent = fs.readFileSync(filePath, encoding);
    const allLines = initialContent.split("\n");
    const initialLines = allLines.slice(-lines - 1, -1); // Remove last empty line

    console.log(`==> ${path.basename(filePath)} <==`);
    initialLines.forEach(printLine);

    // Track file position
    let lastPosition = fileSize;
    let buffer = "";

    // Watch for file changes
    const watcher = fs.watchFile(filePath, { interval: 100 }, (curr, prev) => {
        // Check if file was truncated (rotated)
        if (curr.size < lastPosition) {
            console.log(
                `\n==> File ${path.basename(filePath)} was truncated <==`,
            );
            lastPosition = 0;
            buffer = "";
        }

        // Check if file has new content
        if (curr.size > lastPosition) {
            // Read new content from last position
            const stream = fs.createReadStream(filePath, {
                start: lastPosition,
                encoding: encoding,
            });

            stream.on("data", (chunk: string | Buffer) => {
                // Convert buffer to string if needed
                const chunkStr =
                    typeof chunk === "string"
                        ? chunk
                        : chunk.toString(encoding);

                // Add chunk to buffer
                buffer += chunkStr;

                // Process complete lines
                const lines = buffer.split("\n");

                // Keep the last incomplete line in buffer
                buffer = lines.pop() || "";

                // Print complete lines
                lines.forEach(printLine);
            });

            stream.on("error", (err) => {
                console.error(`Error reading file: ${err.message}`);
            });

            // Update position
            lastPosition = curr.size;
        }
    });

    // Return cleanup function
    return () => {
        fs.unwatchFile(filePath);
        console.log(`\nStopped tailing ${filePath}`);
    };
}

function buf2hex(buffer: Uint8Array) {
    return [...new Uint8Array(buffer)]
        .map((x) => x.toString(16).padStart(2, "0"))
        .join("");
}

async function sleep(ms: number) {
    await new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
    // Show usage if no arguments provided
    if (process.argv.length < 3) {
        console.log(
            "Usage: npx ts-node play_chess_cli.ts <program-path> [ws-endpoint] [log-path]",
        );
        console.log("  program-path: Path to the chess-game.polkavm file");
        console.log(
            "  ws-endpoint:  WebSocket endpoint (default: ws://127.0.0.1:9944)",
        );
        console.log(
            "  log-path:     Optional path to log file for tailing with polkavm grep",
        );
        console.log("");
        console.log("This will start an interactive chess game where you can:");
        console.log("  - Create and join games");
        console.log("  - Make moves as Alice or Bob");
        console.log("  - View game states");
        console.log("");
        console.log("Example:");
        console.log(
            "  npx ts-node play_chess_cli.ts ../output/chess-game.polkavm",
        );
        console.log(
            "  npx ts-node play_chess_cli.ts ../output/chess-game.polkavm ws://127.0.0.1:9944 /tmp/zombie-logs/alice.log",
        );
        process.exit(1);
    }

    // Retrieve the smart contract, WS endpoint, and optional log path from command-line arguments
    const programPath = process.argv[2];
    const wsEndpoint = process.argv[3] || "ws://127.0.0.1:9944";
    const logPath = process.argv[4]; // Optional log file path

    if (!fs.existsSync(programPath)) {
        console.error(`Program file not found: ${programPath}`);
        process.exit(1);
    }

    // Start log tailing if log path is provided
    let logTailProcess: ChildProcess | null = null;
    let cleanupCalled = false;
    let lastLogTime = Date.now();
    if (logPath) {
        if (!fs.existsSync(logPath)) {
            console.warn(
                `Log file not found: ${logPath}. Continuing without log tailing.`,
            );
        } else {
            console.log(`Starting log tail for: ${logPath}`);

            const stopTailing1 = tailWithGrep(logPath, "polkavm");

            // logTailProcess = spawn("tail", ["-f", logPath], { stdio: "pipe" });

            // if (logTailProcess.stdout) {
            //     const grep = spawn("grep", ["polkavm"], {
            //         stdio: ["pipe", "pipe", "inherit"],
            //     });

            //     if (grep.stdout) {
            //         const sed = spawn("sed", ["s/.*polkavm:/polkavm:/"], {
            //             stdio: ["pipe", "pipe", "inherit"],
            //         });

            //         // Monitor log output to track when logs stop
            //         if (sed.stdout) {
            //             sed.stdout.on("data", () => {
            //                 lastLogTime = Date.now();
            //             });
            //             sed.stdout.pipe(process.stdout);
            //         }

            //         grep.stdout.pipe(sed.stdin);
            //     }

            //     logTailProcess.stdout.pipe(grep.stdin);
            // }

            // logTailProcess.on("error", (err) => {
            //     console.warn(`Log tailing error: ${err.message}`);
            // });
        }
    }

    // Cleanup function to stop log tailing
    const cleanup = () => {
        if (logTailProcess && !cleanupCalled) {
            cleanupCalled = true;
            console.log("\nStopping log tail...");
            // logTailProcess.kill();
        }
    };

    // Handle process termination
    process.on("SIGINT", cleanup);
    process.on("SIGTERM", cleanup);
    process.on("exit", cleanup);

    // Connect to the node
    const wsProvider = new WsProvider(wsEndpoint);
    const api = await ApiPromise.create({
        provider: wsProvider,
        noInitWarn: true,
        types: {
            MakeMove: {
                game_id: "u64",
                from_square: "String",
                to_square: "String",
                promotion: "Option<String>",
            },
            ChessCommand: {
                _enum: {
                    CreateGame: null,
                    JoinGame: "u64",
                    MakeMove: "MakeMove",
                    GetGameState: "u64",
                    GetPlayerGames: null,
                },
            },
        },
    });

    // Initialize keyring and add Alice account
    const keyring = new Keyring({ type: "sr25519" });
    const ALICE = keyring.addFromUri("//Alice");
    const BOB = keyring.addFromUri("//Bob");

    console.log(`Caller 0 address: ${ALICE.address}`);
    console.log(`Caller 1 address: ${BOB.address}`);

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

    const gasLimit = 2000000;
    const gasPrice = 10;

    let currentPlayer = ALICE;
    let currentPlayerName = "Alice";

    const execute = async (
        userData: string,
        caller: KeyringPair = currentPlayer,
    ) => {
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
                .signAndSend(caller, ({ events = [], status }) => {
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

        await sleep(500);

        console.log("Program executed successfully!");

        let status = "success";
        if (notEnoughGas.toPrimitive()) {
            status = "out of gas";
        }
        if (trap.toPrimitive()) {
            status = "trap";
        }
        let output = (result as Option<u64>).toHuman();
        let gasSpent =
            (gasBefore as u32).toNumber() - (gasAfter as i64).toNumber();
        console.log(
            `status: ${status}, result: ${output}, gas spent: ${gasSpent}`,
        );
    };

    // Create readline interface for interactive input
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    // Enhanced cleanup function to handle both log tailing and readline
    let isShuttingDown = false;
    const enhancedCleanup = async () => {
        if (isShuttingDown) return;
        isShuttingDown = true;

        console.log("\nShutting down gracefully...");

        // Close readline interface
        rl.close();

        // Stop log tailing
        cleanup();

        // Disconnect from the node if connected
        try {
            await api.disconnect();
            console.log("Disconnected from node");
        } catch (error) {
            console.error("Error disconnecting from node:", error);
        }

        process.exit(0);
    };

    // Handle Ctrl+C and other termination signals
    process.removeAllListeners("SIGINT");
    process.removeAllListeners("SIGTERM");
    process.removeAllListeners("exit");

    process.on("SIGINT", enhancedCleanup);
    process.on("SIGTERM", enhancedCleanup);

    // Handle readline interface close
    rl.on("close", () => {
        if (!isShuttingDown) {
            console.log("\nGoodbye!");
            process.exit(0);
        }
    });

    const askQuestion = (question: string): Promise<string> => {
        return new Promise((resolve) => {
            rl.question(question, resolve);
        });
    };

    const parseCommand = (
        input: string,
    ): { command: string; args: string[] } => {
        const parts = input.trim().split(/\s+/);
        return {
            command: parts[0].toLowerCase(),
            args: parts.slice(1),
        };
    };

    const showHelp = () => {
        console.log("\nAvailable commands:");
        console.log("  create                    - Create a new chess game");
        console.log("  join <game_id>           - Join an existing game");
        console.log(
            "  move <game_id> <from> <to> [promo] - Make a move (e.g., move 1 e2 e4, move 1 e7 e8 Q)",
        );
        console.log("  state <game_id>          - Get game state");
        console.log("  games                    - Get your games");
        console.log(
            "  switch                   - Switch between Alice and Bob",
        );
        console.log("  help                     - Show this help");
        console.log("  quit                     - Exit the program");
        console.log(`\nCurrently playing as: ${currentPlayerName}`);
    };

    const executeCommand = async (input: string): Promise<boolean> => {
        const { command, args } = parseCommand(input);

        try {
            switch (command) {
                case "create":
                    console.log(`Creating new game as ${currentPlayerName}...`);
                    const createCmd = api.createType(
                        "ChessCommand",
                        "CreateGame",
                    );
                    const createData =
                        "0x" + Buffer.from(createCmd.toU8a()).toString("hex");
                    await execute(createData);
                    break;

                case "join":
                    if (args.length !== 1) {
                        console.log("Usage: join <game_id>");
                        break;
                    }
                    const gameId = BigInt(args[0]);
                    console.log(
                        `Joining game ${gameId} as ${currentPlayerName}...`,
                    );
                    const joinCmd = api.createType("ChessCommand", {
                        JoinGame: gameId,
                    });
                    const joinData =
                        "0x" + Buffer.from(joinCmd.toU8a()).toString("hex");
                    await execute(joinData);
                    break;

                case "move":
                    if (args.length < 3) {
                        console.log(
                            "Usage: move <game_id> <from> <to> [promotion]",
                        );
                        console.log("Example: move 1 e2 e4");
                        console.log("Example: move 1 e7 e8 Q");
                        break;
                    }
                    const moveGameId = BigInt(args[0]);
                    const fromSquare = args[1];
                    const toSquare = args[2];
                    const promotion = args[3] || null;

                    console.log(
                        `Making move ${fromSquare} -> ${toSquare} in game ${moveGameId} as ${currentPlayerName}...`,
                    );
                    const moveCmd = api.createType("ChessCommand", {
                        MakeMove: {
                            game_id: moveGameId,
                            from_square: fromSquare,
                            to_square: toSquare,
                            promotion: promotion,
                        },
                    });
                    const moveData =
                        "0x" + Buffer.from(moveCmd.toU8a()).toString("hex");
                    await execute(moveData);
                    break;

                case "state":
                    if (args.length !== 1) {
                        console.log("Usage: state <game_id>");
                        break;
                    }
                    const stateGameId = BigInt(args[0]);
                    console.log(`Getting state for game ${stateGameId}...`);
                    const stateCmd = api.createType("ChessCommand", {
                        GetGameState: stateGameId,
                    });
                    const stateData =
                        "0x" + Buffer.from(stateCmd.toU8a()).toString("hex");
                    await execute(stateData);
                    break;

                case "games":
                    console.log(`Getting games for ${currentPlayerName}...`);
                    const gamesCmd = api.createType(
                        "ChessCommand",
                        "GetPlayerGames",
                    );
                    const gamesData =
                        "0x" + Buffer.from(gamesCmd.toU8a()).toString("hex");
                    await execute(gamesData);
                    break;

                case "switch":
                    if (currentPlayer === ALICE) {
                        currentPlayer = BOB;
                        currentPlayerName = "Bob";
                    } else {
                        currentPlayer = ALICE;
                        currentPlayerName = "Alice";
                    }
                    console.log(`Switched to ${currentPlayerName}`);
                    break;

                case "help":
                    showHelp();
                    break;

                case "quit":
                case "exit":
                    console.log("Goodbye!");
                    await enhancedCleanup();
                    return false;

                default:
                    console.log(`Unknown command: ${command}`);
                    console.log("Type 'help' for available commands.");
                    break;
            }
        } catch (error) {
            console.error(`Error executing command: ${error}`);
        }

        return true;
    };

    // Interactive loop
    console.log("\n=== Interactive Chess CLI ===");
    showHelp();

    let continueRunning = true;
    while (continueRunning && !isShuttingDown) {
        try {
            const input = await askQuestion(`\n[${currentPlayerName}] > `);
            continueRunning = await executeCommand(input);
        } catch (error) {
            if (isShuttingDown) {
                break;
            }
            console.error("Error reading input:", error);
            break;
        }
    }

    // If we exit the loop normally (not via Ctrl+C), clean up
    if (!isShuttingDown) {
        await enhancedCleanup();
    }
}

main().catch(console.error);
