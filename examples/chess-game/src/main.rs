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

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::str::FromStr;
use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::{AccountId32, Ss58Codec};

qf_polkavm_sdk::safe_api!();

/// Chess piece types
#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// Chess piece colors
#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Chess piece
#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

/// Chess square coordinates (0-7, 0-7)
#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub file: u8, // 0-7 (a-h)
    pub rank: u8, // 0-7 (1-8)
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Option<Square> {
        if file < 8 && rank < 8 {
            Some(Square { file, rank })
        } else {
            None
        }
    }

    pub fn from_str(s: &str) -> Option<Square> {
        if s.len() != 2 {
            return None;
        }
        let bytes = s.as_bytes();
        let file = bytes[0];
        let rank = bytes[1];

        if file >= b'a' && file <= b'h' && rank >= b'1' && rank <= b'8' {
            Some(Square {
                file: file - b'a',
                rank: rank - b'1',
            })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        let file_char = (b'a' + self.file) as char;
        let rank_char = (b'1' + self.rank) as char;
        format!("{}{}", file_char, rank_char)
    }
}

/// Chess move
#[derive(Encode, Decode, Clone, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

/// Chess board state
#[derive(Encode, Decode, Clone, Debug)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
            to_move: Color::White,
            castling_rights: CastlingRights {
                white_kingside: true,
                white_queenside: true,
                black_kingside: true,
                black_queenside: true,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        // Set up initial pieces
        board.setup_initial_position();
        board
    }

    pub fn print_unicode(&self) {
        call_print("  a b c d e f g h");
        for rank in (0..8).rev() {
            let mut line = format!("{} ", rank + 1);
            for file in 0..8 {
                let piece_char = match &self.squares[rank][file] {
                    Some(piece) => match (piece.color, piece.piece_type) {
                        (Color::White, PieceType::King) => "♔",
                        (Color::White, PieceType::Queen) => "♕",
                        (Color::White, PieceType::Rook) => "♖",
                        (Color::White, PieceType::Bishop) => "♗",
                        (Color::White, PieceType::Knight) => "♘",
                        (Color::White, PieceType::Pawn) => "♙",
                        (Color::Black, PieceType::King) => "♚",
                        (Color::Black, PieceType::Queen) => "♛",
                        (Color::Black, PieceType::Rook) => "♜",
                        (Color::Black, PieceType::Bishop) => "♝",
                        (Color::Black, PieceType::Knight) => "♞",
                        (Color::Black, PieceType::Pawn) => "♟",
                    },
                    None => "·",
                };
                line.push_str(piece_char);
                line.push(' ');
            }
            call_print(&line);
        }
        call_print("");
    }

    fn setup_initial_position(&mut self) {
        // White pieces (rank 0)
        self.squares[0][0] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });
        self.squares[1][0] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });
        self.squares[2][0] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        self.squares[3][0] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        });
        self.squares[4][0] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        });
        self.squares[5][0] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        self.squares[6][0] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });
        self.squares[7][0] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });

        for i in 0..8 {
            self.squares[i][1] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
        }

        // Black pieces
        self.squares[0][7] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });
        self.squares[1][7] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });
        self.squares[2][7] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });
        self.squares[3][7] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        });
        self.squares[4][7] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        });
        self.squares[5][7] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });
        self.squares[6][7] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });
        self.squares[7][7] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });

        for i in 0..8 {
            self.squares[i][6] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            });
        }
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square.file as usize][square.rank as usize]
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square.file as usize][square.rank as usize] = piece;
    }

    pub fn is_valid_move(&self, mv: &Move) -> bool {
        // Basic validation
        if let Some(piece) = self.get_piece(mv.from) {
            if piece.color != self.to_move {
                return false;
            }

            // Check if destination has own piece
            if let Some(dest_piece) = self.get_piece(mv.to) {
                if dest_piece.color == piece.color {
                    return false;
                }
            }

            // Piece-specific move validation
            match piece.piece_type {
                PieceType::Pawn => self.is_valid_pawn_move(&mv, &piece),
                PieceType::Rook => self.is_valid_rook_move(&mv),
                PieceType::Knight => self.is_valid_knight_move(&mv),
                PieceType::Bishop => self.is_valid_bishop_move(&mv),
                PieceType::Queen => self.is_valid_queen_move(&mv),
                PieceType::King => self.is_valid_king_move(&mv),
            }
        } else {
            false
        }
    }

    fn is_valid_pawn_move(&self, mv: &Move, piece: &Piece) -> bool {
        let direction = if piece.color == Color::White { 1 } else { -1 };
        let start_rank = if piece.color == Color::White { 1 } else { 6 };

        let file_diff = (mv.to.file as i8) - (mv.from.file as i8);
        let rank_diff = (mv.to.rank as i8) - (mv.from.rank as i8);

        // Forward move
        if file_diff == 0 {
            if rank_diff == direction && self.get_piece(mv.to).is_none() {
                return true;
            }
            // Two squares from start
            if mv.from.rank == start_rank
                && rank_diff == 2 * direction
                && self.get_piece(mv.to).is_none()
            {
                return true;
            }
        }
        // Capture
        else if file_diff.abs() == 1 && rank_diff == direction {
            if self.get_piece(mv.to).is_some() {
                return true;
            }
        }

        false
    }

    fn is_valid_rook_move(&self, mv: &Move) -> bool {
        let file_diff = (mv.to.file as i8) - (mv.from.file as i8);
        let rank_diff = (mv.to.rank as i8) - (mv.from.rank as i8);

        if file_diff == 0 || rank_diff == 0 {
            self.is_path_clear(mv.from, mv.to)
        } else {
            false
        }
    }

    fn is_valid_knight_move(&self, mv: &Move) -> bool {
        let file_diff = ((mv.to.file as i8) - (mv.from.file as i8)).abs();
        let rank_diff = ((mv.to.rank as i8) - (mv.from.rank as i8)).abs();

        (file_diff == 2 && rank_diff == 1) || (file_diff == 1 && rank_diff == 2)
    }

    fn is_valid_bishop_move(&self, mv: &Move) -> bool {
        let file_diff = ((mv.to.file as i8) - (mv.from.file as i8)).abs();
        let rank_diff = ((mv.to.rank as i8) - (mv.from.rank as i8)).abs();

        if file_diff == rank_diff && file_diff > 0 {
            self.is_path_clear(mv.from, mv.to)
        } else {
            false
        }
    }

    fn is_valid_queen_move(&self, mv: &Move) -> bool {
        self.is_valid_rook_move(mv) || self.is_valid_bishop_move(mv)
    }

    fn is_valid_king_move(&self, mv: &Move) -> bool {
        let file_diff = ((mv.to.file as i8) - (mv.from.file as i8)).abs();
        let rank_diff = ((mv.to.rank as i8) - (mv.from.rank as i8)).abs();

        file_diff <= 1 && rank_diff <= 1 && (file_diff > 0 || rank_diff > 0)
    }

    fn is_path_clear(&self, from: Square, to: Square) -> bool {
        let file_diff = (to.file as i8) - (from.file as i8);
        let rank_diff = (to.rank as i8) - (from.rank as i8);

        let file_step = if file_diff == 0 {
            0
        } else {
            file_diff / file_diff.abs()
        };
        let rank_step = if rank_diff == 0 {
            0
        } else {
            rank_diff / rank_diff.abs()
        };

        let mut current_file = from.file as i8 + file_step;
        let mut current_rank = from.rank as i8 + rank_step;

        while current_file != to.file as i8 || current_rank != to.rank as i8 {
            if let Some(square) = Square::new(current_file as u8, current_rank as u8) {
                if self.get_piece(square).is_some() {
                    return false;
                }
            }
            current_file += file_step;
            current_rank += rank_step;
        }

        true
    }

    pub fn make_move(&mut self, mv: &Move) -> bool {
        if !self.is_valid_move(mv) {
            return false;
        }

        let piece = self.get_piece(mv.from).unwrap();

        // Handle promotion
        let final_piece = if let Some(promotion) = mv.promotion {
            Piece {
                piece_type: promotion,
                color: piece.color,
            }
        } else {
            piece
        };

        // Make the move
        self.set_piece(mv.from, None);
        self.set_piece(mv.to, Some(final_piece));

        // Switch turns
        self.to_move = self.to_move.opposite();

        if self.to_move == Color::White {
            self.fullmove_number += 1;
        }

        true
    }

    pub fn find_king(&self, color: Color) -> Option<Square> {
        for file in 0..8 {
            for rank in 0..8 {
                if let Some(piece) = self.squares[file][rank] {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Square::new(file as u8, rank as u8);
                    }
                }
            }
        }
        None
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_square) = self.find_king(color) {
            // Check if any opponent piece can attack the king
            for file in 0..8 {
                for rank in 0..8 {
                    if let Some(piece) = self.squares[file][rank] {
                        if piece.color != color {
                            let from = Square::new(file as u8, rank as u8).unwrap();
                            let test_move = Move {
                                from,
                                to: king_square,
                                promotion: None,
                            };

                            // Temporarily change turn to test if move is valid
                            let original_turn = self.to_move;
                            let mut temp_board = self.clone();
                            temp_board.to_move = piece.color;

                            if temp_board.is_valid_move(&test_move) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn has_legal_moves(&self, color: Color) -> bool {
        for from_file in 0..8 {
            for from_rank in 0..8 {
                if let Some(piece) = self.squares[from_file][from_rank] {
                    if piece.color == color {
                        let from = Square::new(from_file as u8, from_rank as u8).unwrap();

                        for to_file in 0..8 {
                            for to_rank in 0..8 {
                                let to = Square::new(to_file as u8, to_rank as u8).unwrap();
                                let test_move = Move {
                                    from,
                                    to,
                                    promotion: None,
                                };

                                if self.is_valid_move(&test_move) {
                                    // Test if move leaves king in check
                                    let mut temp_board = self.clone();
                                    temp_board.make_move(&test_move);

                                    if !temp_board.is_in_check(color) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_in_check(self.to_move) && !self.has_legal_moves(self.to_move)
    }

    pub fn is_stalemate(&self) -> bool {
        !self.is_in_check(self.to_move) && !self.has_legal_moves(self.to_move)
    }

    pub fn is_insufficient_material(&self) -> bool {
        let mut piece_count = 0;
        let mut has_major_piece = false;

        for file in 0..8 {
            for rank in 0..8 {
                if let Some(piece) = self.squares[file][rank] {
                    piece_count += 1;
                    match piece.piece_type {
                        PieceType::Queen | PieceType::Rook | PieceType::Pawn => {
                            has_major_piece = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        // King vs King, or King+minor vs King
        piece_count <= 3 && !has_major_piece
    }
}

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

/// Game status enumeration
#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum GameStatus {
    WaitingForPlayer,
    InProgress,
    WhiteWins,
    BlackWins,
    Draw,
}

/// Main chess game structure
#[derive(Encode, Decode, Clone, Debug)]
pub struct ChessGame {
    pub game_id: u64,
    pub white_player: Option<AccountId32>,
    pub black_player: Option<AccountId32>,
    pub status: GameStatus,
    pub board: Board,
}

/// Game counter to track total games created
#[derive(Encode, Decode, Default)]
struct GameCounter {
    count: u64,
}

impl ChessGame {
    /// Create a new chess game with initial board setup
    pub fn new(game_id: u64, creator: AccountId32) -> Self {
        ChessGame {
            game_id,
            white_player: Some(creator),
            black_player: None,
            status: GameStatus::WaitingForPlayer,
            board: Board::new(),
        }
    }

    /// Try to join the game as the second player
    pub fn join_game(&mut self, player_address: AccountId32) -> Result<(), &'static str> {
        if self.black_player.is_some() {
            return Err("Game already has two players");
        }

        if Some(player_address.clone()) == self.white_player {
            return Err("Cannot join your own game");
        }

        self.black_player = Some(player_address);
        self.status = GameStatus::InProgress;
        Ok(())
    }

    pub fn make_move(
        &mut self,
        from_square: String,
        to_square: String,
        promotion: Option<String>,
    ) -> Result<(), &'static str> {
        let player = get_caller_address();

        // Verify it's the player's turn
        let current_color = self.board.to_move;
        let player_is_white = Some(player.clone()) == self.white_player;
        let player_is_black = Some(player) == self.black_player;

        if (current_color == Color::White && !player_is_white)
            || (current_color == Color::Black && !player_is_black)
        {
            return Err("Not your turn");
        }

        // Verify game is in progress
        if self.status != GameStatus::InProgress {
            return Err("Game is not in progress");
        }

        // Parse squares
        let from = Square::from_str(&from_square).ok_or("Invalid from square")?;
        let to = Square::from_str(&to_square).ok_or("Invalid to square")?;

        // Parse promotion
        let promotion_type = if let Some(promo) = promotion.as_ref() {
            match promo.as_str() {
                "Q" => Some(PieceType::Queen),
                "R" => Some(PieceType::Rook),
                "B" => Some(PieceType::Bishop),
                "N" => Some(PieceType::Knight),
                _ => return Err("Invalid promotion piece"),
            }
        } else {
            None
        };

        let chess_move = Move {
            from,
            to,
            promotion: promotion_type,
        };

        // Make the move
        if !self.board.make_move(&chess_move) {
            return Err("Invalid move");
        }

        // Check for game end conditions
        if self.board.is_checkmate() {
            self.status = if current_color == Color::White {
                GameStatus::BlackWins
            } else {
                GameStatus::WhiteWins
            };
        } else if self.board.is_stalemate() || self.board.is_insufficient_material() {
            self.status = GameStatus::Draw;
        }

        call_print(&format!("Move made: {} -> {}", from_square, to_square));

        if self.status != GameStatus::InProgress {
            call_print(&format!("Game ended: {:?}", self.status));
        }

        Ok(())
    }
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
    // Load the game
    let mut game = StorageCell::<ChessGame>::load_from_key(format!("game_{game_id}").as_str())
        .unwrap()
        .unwrap();

    match game.data.make_move(from_square, to_square, promotion) {
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
    game.board.print_unicode();

    0 // Success
}

/// Get games for the current player
fn get_player_games() -> u64 {
    let caller_address = get_caller_address();
    call_print(&format!(
        "Player {} games - feature not yet implemented",
        caller_address.to_ss58check()
    ));
    0
}

fn get_caller_address() -> AccountId32 {
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
fn call_print(msg: &str) -> u64 {
    unsafe { print(msg.as_ptr() as u32, msg.len() as u32) }
}
