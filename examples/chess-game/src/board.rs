use alloc::{format, string::String};
use parity_scale_codec::{Decode, Encode};

use crate::*;

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
#[derive(Encode, Decode, Clone, Copy, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

/// Chess board state
#[derive(Encode, Decode, Clone, Copy, Debug)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

#[derive(Encode, Decode, Clone, Copy, Debug)]
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

    pub fn print_unicode(&self, mut call_print: impl FnMut(&str) -> u64) {
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
                line.push_str(&format!("{piece_char} "));
            }
            call_print(&line);
        }
        call_print("");
    }

    fn setup_initial_position(&mut self) {
        let piece_order = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        // White pieces on rank 1 (index 0)
        for (file, &piece_type) in piece_order.iter().enumerate() {
            self.squares[0][file] = Some(Piece {
                piece_type,
                color: Color::White,
            });
        }

        // White pawns on rank 2 (index 1)
        for file in 0..8 {
            self.squares[1][file] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
        }

        // Black pawns on rank 7 (index 6)
        for file in 0..8 {
            self.squares[6][file] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            });
        }

        // Black pieces on rank 8 (index 7)
        for (file, &piece_type) in piece_order.iter().enumerate() {
            self.squares[7][file] = Some(Piece {
                piece_type,
                color: Color::Black,
            });
        }
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square.rank as usize][square.file as usize]
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square.rank as usize][square.file as usize] = piece;
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
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
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
            for rank in 0..8 {
                for file in 0..8 {
                    if let Some(piece) = self.squares[rank][file] {
                        if piece.color != color {
                            let from = Square::new(file as u8, rank as u8).unwrap();
                            let test_move = Move {
                                from,
                                to: king_square,
                                promotion: None,
                            };

                            // Temporarily change turn to test if move is valid
                            let mut temp_board = *self;
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
        for from_rank in 0..8 {
            for from_file in 0..8 {
                if let Some(piece) = self.squares[from_rank][from_file] {
                    if piece.color == color {
                        let from = Square::new(from_file as u8, from_rank as u8).unwrap();

                        for to_rank in 0..8 {
                            for to_file in 0..8 {
                                let to = Square::new(to_file as u8, to_rank as u8).unwrap();
                                let test_move = Move {
                                    from,
                                    to,
                                    promotion: None,
                                };

                                if self.is_valid_move(&test_move) {
                                    // Test if this move leaves king in check
                                    let mut temp_board = *self;
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

        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
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
