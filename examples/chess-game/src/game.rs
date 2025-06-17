use alloc::{format, string::String};
use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::AccountId32;

use crate::board::{Board, Color, Move, PieceType, Square};

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
        player: AccountId32,
        from_square: String,
        to_square: String,
        promotion: Option<String>,
        mut call_print: impl FnMut(&str) -> u64,
    ) -> Result<(), &'static str> {
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

        call_print("");
        self.board.print_unicode(&mut call_print);

        // TODO: add print for other statuses
        if self.status != GameStatus::InProgress {
            call_print(&format!("Game ended: {:?}", self.status));
        }

        Ok(())
    }
}
