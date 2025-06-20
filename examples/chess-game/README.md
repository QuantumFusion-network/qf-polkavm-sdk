# Chess Game Smart Contract

A simple chess game smart contract prototype built with the QF PolkaVM SDK.

<p align="center">
  <img src="./demo.gif" alt="Demo of the gameplay" width="600"/>
</p>

## Features

This initial prototype includes:
- ✅ Create a new chess game
- ✅ Join an existing game as the second player
- ✅ Get current game state with board setup
- ✅ Initial chess board with standard piece positioning
- ✅ Player management and turn tracking
- ✅ Move validation and execution
- ✅ Check/checkmate detection
- ✅ Special moves (castling, en passant, pawn promotion)

## Planned Features (Future)
- Game history and move tracking
- Draw conditions

## Building

From the project root:

```console
./build_polkavm.sh chess-game
```

This will create `output/chess-game.polkavm` ready for deployment.

## Usage

Run a local node and run:
```console
cd examples/chess-game/play-chess-cli/

npx ts-node play_chess_cli.ts ../../../output/chess-game.polkavm ws://127.0.0.1:9944 /tmp/zombie-logs/alice.log
```

The contract accepts SCALE-encoded commands. Here are the available commands:

### 1. Create Game
Creates a new chess game with the caller as the white player.

```rust
ChessCommand::CreateGame
```

### 2. Join Game
Join an existing game as the black player.

```rust
ChessCommand::JoinGame(game_id)
```

### 3. Get Game State
Retrieve the current state of a game.

```rust
ChessCommand::GetGameState(game_id)
```

## Example Usage with CLI

1. **Deploy the contract** using the QF Network Portal
2. **Create a game:**
   ```bash
   # Encode CreateGame command
   echo '0' | # CreateGame is variant 0
   npx ts-node uploadAndExecute.ts wss://localhost:9944 ../output/chess-game.polkavm
   ```

3. **Join a game:**
   ```bash
   # Encode JoinGame(1) command
   echo '{"JoinGame": 1}' | # Join game with ID 1
   npx ts-node uploadAndExecute.ts wss://localhost:9944 ../output/chess-game.polkavm
   ```

4. **Get game state:**
   ```bash
   # Encode GetGameState(1) command
   echo '{"GetGameState": 1}' |
   npx ts-node uploadAndExecute.ts wss://localhost:9944 ../output/chess-game.polkavm
   ```

## Game State Structure

```rust
pub struct ChessGame {
    pub game_id: u64,
    pub white_player: Option<u64>,  // Address index of white player
    pub black_player: Option<u64>,  // Address index of black player
    pub status: GameStatus,         // WaitingForPlayer, InProgress, etc.
    pub board: Board,               // Current board state with move tracking
}
```

## Game Flow

1. **Player A creates a game** → Game created with Player A as white, waiting for black player
2. **Player B joins the game** → Game status changes to "InProgress", both players assigned
3. **Players can query game state** → View board, current turn, player assignments

## Error Codes

- `0`: Success
- `1`: Failed to decode command
- `2`: Failed to join game (already full or trying to join own game)
- `3`: Failed to save game counter
- `4`: Game not found
- `5`: Failed to decode game data
- `6`: Game data too large for storage

## Board Representation

The board is represented as an 8x8 array where:
- `[0][0]` is bottom-left (a1 in chess notation)
- `[7][7]` is top-right (h8 in chess notation)
- White pieces start on rows 0-1
- Black pieces start on rows 6-7

## Storage Keys

- `"game_counter"`: Global counter for game IDs
- `"game_{id}"`: Individual game data

## Testing

To run unit tests:
```console
cargo +nightly test --package chess-game
```

Deploy the contract and test the basic flow:

1. Create a game and note the game ID returned
2. Use a different account to join the game
3. Query the game state to see both players assigned
4. Verify the initial board setup is correct

## Next Steps

This prototype demonstrates the basic game management. The next development phase would add:
- Move input and validation
- Turn enforcement
- Win condition detection
- Move history tracking
