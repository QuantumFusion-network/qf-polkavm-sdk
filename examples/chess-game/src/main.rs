//! Simple Chess Game Smart Contract Prototype
//!
//! Features:
//! - Create a new chess game
//! - Join an existing game as second player
//! - Get current game state
//! - Make a move
//!
//! This is a minimal prototype focusing on game management basics.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::{AccountId32, Ss58Codec};

pub mod board;
pub mod game;

use game::ChessGame;

qf_polkavm_sdk::safe_api!();

/// Commands that can be sent to the chess contract
#[derive(Encode, Decode, Debug)]
pub enum ChessCommand {
    CreateGame,
    JoinGame(u64),
    MakeMove {
        game_id: u64,
        from_square: String,
        to_square: String,
        promotion: Option<String>,
    },
    GetGameState(u64),
    GetPlayerGames,
}

/// Game counter to track total games created
#[derive(Encode, Decode, Default)]
struct GameCounter {
    count: u64,
}

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let command: ChessCommand = load_data().unwrap();
    call_print(&format!("Received command: {command:?}"));

    match command {
        ChessCommand::CreateGame => create_game(),
        ChessCommand::JoinGame(game_id) => join_game(game_id),
        ChessCommand::MakeMove {
            game_id,
            from_square,
            to_square,
            promotion,
        } => make_move(game_id, from_square, to_square, promotion),
        ChessCommand::GetGameState(game_id) => get_game_state(game_id),
        ChessCommand::GetPlayerGames => get_player_games(),
    }
}

/// Create a new chess game
fn create_game() -> u64 {
    let caller_address = get_caller_address();

    // Get current game counter
    let mut counter = StorageCell::<GameCounter>::load_from_key("game_counter")
        .unwrap()
        .unwrap_or_else(|| StorageCell {
            data: GameCounter::default(),
            storage_key: try_into_key("game_counter").unwrap(),
        });
    counter.data.count += 1;
    let game_id = counter.data.count;
    // Save game counter
    counter.save().unwrap();

    // Create new game
    let game = StorageCell {
        data: ChessGame::new(game_id, caller_address.clone()),
        storage_key: try_into_key(&format!("game_{game_id}")).unwrap(),
    };
    game.save().unwrap();

    call_print(&format!(
        "Created game {game_id} for player {}",
        caller_address.to_ss58check()
    ));

    // Return game ID in the result
    game_id
}

/// Join an existing game
fn join_game(game_id: u64) -> u64 {
    let caller_address = get_caller_address();

    // Load the game
    let mut game = StorageCell::<ChessGame>::load_from_key(format!("game_{game_id}").as_str())
        .unwrap()
        .unwrap();

    // Try to join the game
    match game.data.join_game(caller_address.clone()) {
        Ok(()) => {
            call_print(&format!(
                "Player {} joined game {}",
                caller_address.to_ss58check(),
                game_id
            ));

            // Save the updated game
            game.save().unwrap();

            0 // Success
        }
        Err(msg) => {
            call_print(&format!("Failed to join game: {}", msg));
            2 // Join failed
        }
    }
}

fn make_move(
    game_id: u64,
    from_square: String,
    to_square: String,
    promotion: Option<String>,
) -> u64 {
    let caller_address = get_caller_address();

    // Load the game
    let mut game = StorageCell::<ChessGame>::load_from_key(format!("game_{game_id}").as_str())
        .unwrap()
        .unwrap();

    match game.data.make_move(
        caller_address,
        from_square,
        to_square,
        promotion,
        call_print,
    ) {
        Ok(()) => {
            game.save().unwrap();
            0 // Success
        }
        Err(msg) => {
            call_print(&format!("Move failed: {}", msg));
            1 // Move failed
        }
    }
}

/// Get the current state of a game
fn get_game_state(game_id: u64) -> u64 {
    // Load the game
    let game = StorageCell::<ChessGame>::load_from_key(format!("game_{game_id}").as_str())
        .unwrap()
        .unwrap()
        .data;

    // Print game state information
    call_print(&format!("=== Game {} State ===", game.game_id));
    call_print(&format!("Status: {:?}", game.status));
    call_print(&format!(
        "White Player: {:?}",
        game.white_player.map(|a| a.to_ss58check())
    ));
    call_print(&format!(
        "Black Player: {:?}",
        game.black_player.map(|a| a.to_ss58check())
    ));
    call_print(&format!("Current Turn: {:?}", game.board.to_move));
    call_print(&format!("Move Count: {}", game.board.fullmove_number));
    call_print("");

    // Print the chess board
    game.board.print_unicode(call_print);

    0 // Success
}

/// Get games for the current player
fn get_player_games() -> u64 {
    // TODO: implement feature
    let caller_address = get_caller_address();
    call_print(&format!(
        "Player {} games - feature not yet implemented",
        caller_address.to_ss58check()
    ));
    0
}

pub fn get_caller_address() -> AccountId32 {
    // Get caller index
    let id = unsafe { caller() } as u32;

    // Get length of SCALE encoded address in bytes
    let len = unsafe { get_address_len(id) } as usize;

    // Allocate buffer to read
    let mut address_buffer: Vec<u8> = Vec::with_capacity(len);
    address_buffer.resize(len, 0);

    // Call host function to load encoded address
    if unsafe { get_address(id, address_buffer.as_mut_ptr() as u32) } != 0 {
        panic!();
    }

    // Decode SCALE into address
    let account_id = AccountId32::decode(&mut &address_buffer[..]).unwrap();
    account_id
}

/// Print message to runtime logs
pub fn call_print(msg: &str) -> u64 {
    unsafe { print(msg.as_ptr() as u32, msg.len() as u32) }
}
