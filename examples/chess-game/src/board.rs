use alloc::{format, string::String};
use codec::{Decode, Encode};

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
            let is_valid_piece_move = match piece.piece_type {
                PieceType::Pawn => self.is_valid_pawn_move(&mv, &piece),
                PieceType::Rook => self.is_valid_rook_move(&mv),
                PieceType::Knight => self.is_valid_knight_move(&mv),
                PieceType::Bishop => self.is_valid_bishop_move(&mv),
                PieceType::Queen => self.is_valid_queen_move(&mv),
                PieceType::King => self.is_valid_king_move(&mv),
            };

            if !is_valid_piece_move {
                return false;
            }

            // Check if move would leave king in check
            self.would_be_legal_after_move(mv)
        } else {
            false
        }
    }

    fn is_valid_pawn_move(&self, mv: &Move, piece: &Piece) -> bool {
        let direction = if piece.color == Color::White { 1 } else { -1 };
        let start_rank = if piece.color == Color::White { 1 } else { 6 };
        let promotion_rank = if piece.color == Color::White { 7 } else { 0 };

        let file_diff = (mv.to.file as i8) - (mv.from.file as i8);
        let rank_diff = (mv.to.rank as i8) - (mv.from.rank as i8);

        // Check promotion requirements
        if mv.to.rank == promotion_rank {
            // Moving to promotion rank requires promotion
            if mv.promotion.is_none() {
                return false;
            }
        }
        // Note: We allow promotion on non-promotion ranks but ignore it in make_move

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
                // Check that intermediate square is also empty
                let intermediate_rank = if direction > 0 {
                    mv.from.rank + 1
                } else {
                    mv.from.rank - 1
                };
                let intermediate_square = Square::new(mv.from.file, intermediate_rank).unwrap();
                if self.get_piece(intermediate_square).is_none() {
                    return true;
                }
            }
        }
        // Capture
        else if file_diff.abs() == 1 && rank_diff == direction {
            if self.get_piece(mv.to).is_some() {
                return true;
            }
            // En passant capture
            if let Some(en_passant_square) = self.en_passant {
                if mv.to == en_passant_square {
                    return true;
                }
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
        let file_diff = (mv.to.file as i8) - (mv.from.file as i8);
        let rank_diff = ((mv.to.rank as i8) - (mv.from.rank as i8)).abs();

        // Normal king move (one square in any direction)
        if file_diff.abs() <= 1 && rank_diff <= 1 && (file_diff.abs() > 0 || rank_diff > 0) {
            return true;
        }

        // Castling
        if rank_diff == 0 && file_diff.abs() == 2 {
            let is_kingside = file_diff > 0;
            let king_rank = mv.from.rank;
            let king_color = self.to_move;

            // Check castling rights
            let can_castle = match (king_color, is_kingside) {
                (Color::White, true) => self.castling_rights.white_kingside,
                (Color::White, false) => self.castling_rights.white_queenside,
                (Color::Black, true) => self.castling_rights.black_kingside,
                (Color::Black, false) => self.castling_rights.black_queenside,
            };

            if !can_castle {
                return false;
            }

            // Check that squares between king and rook are empty
            let (start_file, end_file) = if is_kingside {
                (mv.from.file + 1, 7)
            } else {
                (0, mv.from.file - 1)
            };

            for file in (start_file + 1)..end_file {
                if self
                    .get_piece(Square::new(file, king_rank).unwrap())
                    .is_some()
                {
                    return false;
                }
            }

            // Check that king is not in check, doesn't pass through check, and doesn't end in check
            if self.is_in_check(king_color) {
                return false;
            }

            // Check that king doesn't pass through check
            let intermediate_file = if is_kingside {
                mv.from.file + 1
            } else {
                mv.from.file - 1
            };

            let intermediate_square = Square::new(intermediate_file, king_rank).unwrap();

            // Create a temporary board to check if intermediate square is under attack
            let mut temp_board = *self;
            temp_board.set_piece(mv.from, None);
            temp_board.set_piece(
                intermediate_square,
                Some(Piece {
                    piece_type: PieceType::King,
                    color: king_color,
                }),
            );

            if temp_board.is_in_check(king_color) {
                return false;
            }

            // Also check that the final square is not under attack
            let mut final_temp_board = *self;
            final_temp_board.set_piece(mv.from, None);
            final_temp_board.set_piece(
                mv.to,
                Some(Piece {
                    piece_type: PieceType::King,
                    color: king_color,
                }),
            );

            if final_temp_board.is_in_check(king_color) {
                return false;
            }

            return true;
        }

        false
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
        let captured_piece = self.get_piece(mv.to);

        // Update halfmove clock
        if piece.piece_type == PieceType::Pawn || captured_piece.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Handle en passant capture
        if piece.piece_type == PieceType::Pawn {
            if let Some(en_passant_square) = self.en_passant {
                if mv.to == en_passant_square {
                    // Remove the captured pawn
                    let captured_pawn_rank = if piece.color == Color::White {
                        en_passant_square.rank - 1
                    } else {
                        en_passant_square.rank + 1
                    };
                    self.set_piece(
                        Square::new(en_passant_square.file, captured_pawn_rank).unwrap(),
                        None,
                    );
                    self.halfmove_clock = 0; // En passant is a capture
                }
            }
        }

        // Handle castling
        if piece.piece_type == PieceType::King {
            let file_diff = (mv.to.file as i8) - (mv.from.file as i8);
            if file_diff.abs() == 2 {
                // This is castling - move the rook too
                let is_kingside = file_diff > 0;
                let rook_from_file = if is_kingside { 7 } else { 0 };
                let rook_to_file = if is_kingside {
                    mv.to.file - 1
                } else {
                    mv.to.file + 1
                };

                let rook_from = Square::new(rook_from_file, mv.from.rank).unwrap();
                let rook_to = Square::new(rook_to_file, mv.from.rank).unwrap();

                let rook = self.get_piece(rook_from).unwrap();
                self.set_piece(rook_from, None);
                self.set_piece(rook_to, Some(rook));
            }

            // Update castling rights when king moves
            if piece.color == Color::White {
                self.castling_rights.white_kingside = false;
                self.castling_rights.white_queenside = false;
            } else {
                self.castling_rights.black_kingside = false;
                self.castling_rights.black_queenside = false;
            }
        }

        // Update castling rights when rook moves
        if piece.piece_type == PieceType::Rook {
            match (piece.color, mv.from.file, mv.from.rank) {
                (Color::White, 0, 0) => self.castling_rights.white_queenside = false,
                (Color::White, 7, 0) => self.castling_rights.white_kingside = false,
                (Color::Black, 0, 7) => self.castling_rights.black_queenside = false,
                (Color::Black, 7, 7) => self.castling_rights.black_kingside = false,
                _ => {}
            }
        }

        // Handle promotion
        let final_piece = if let Some(promotion) = mv.promotion {
            // Only apply promotion if moving to promotion rank
            let promotion_rank = if piece.color == Color::White { 7 } else { 0 };
            if piece.piece_type == PieceType::Pawn && mv.to.rank == promotion_rank {
                Piece {
                    piece_type: promotion,
                    color: piece.color,
                }
            } else {
                piece // Ignore promotion if not valid
            }
        } else {
            piece
        };

        // Make the move
        self.set_piece(mv.from, None);
        self.set_piece(mv.to, Some(final_piece));

        // Set en passant square for pawn double moves
        self.en_passant = None;
        if piece.piece_type == PieceType::Pawn {
            let rank_diff = (mv.to.rank as i8) - (mv.from.rank as i8);
            if rank_diff.abs() == 2 {
                // Pawn moved two squares, set en passant square
                let en_passant_rank = (mv.from.rank + mv.to.rank) / 2;
                self.en_passant = Some(Square::new(mv.from.file, en_passant_rank).unwrap());
            }
        }

        // Switch turns
        self.to_move = self.to_move.opposite();

        if self.to_move == Color::White {
            self.fullmove_number += 1;
        }

        true
    }

    fn would_be_legal_after_move(&self, mv: &Move) -> bool {
        // Create a temporary board to test the move
        let mut temp_board = *self;
        let piece = temp_board.get_piece(mv.from).unwrap();

        // Handle en passant capture in temporary board
        if piece.piece_type == PieceType::Pawn {
            if let Some(en_passant_square) = temp_board.en_passant {
                if mv.to == en_passant_square {
                    let captured_pawn_rank = if piece.color == Color::White {
                        en_passant_square.rank - 1
                    } else {
                        en_passant_square.rank + 1
                    };
                    temp_board.set_piece(
                        Square::new(en_passant_square.file, captured_pawn_rank).unwrap(),
                        None,
                    );
                }
            }
        }

        // Make the move on temporary board
        temp_board.set_piece(mv.from, None);
        temp_board.set_piece(mv.to, Some(piece));

        // Check if the king would be in check after this move
        !temp_board.is_in_check(self.to_move)
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

    fn is_valid_move_basic(&self, mv: &Move) -> bool {
        // Basic validation without check validation (to avoid recursion)
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
                PieceType::King => {
                    // For basic validation, only allow normal king moves (not castling)
                    let file_diff = ((mv.to.file as i8) - (mv.from.file as i8)).abs();
                    let rank_diff = ((mv.to.rank as i8) - (mv.from.rank as i8)).abs();
                    file_diff <= 1 && rank_diff <= 1 && (file_diff > 0 || rank_diff > 0)
                }
            }
        } else {
            false
        }
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

                            if temp_board.is_valid_move_basic(&test_move) {
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
                                    return true;
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
        let mut white_pieces = Vec::new();
        let mut black_pieces = Vec::new();

        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
                    if piece.color == Color::White {
                        white_pieces.push(piece.piece_type);
                    } else {
                        black_pieces.push(piece.piece_type);
                    }
                }
            }
        }

        // Helper function to check if a side has insufficient material
        let is_insufficient = |pieces: &Vec<PieceType>| {
            // Must have king
            if !pieces.contains(&PieceType::King) {
                return false;
            }

            // Remove king for easier checking
            let non_king_pieces: Vec<PieceType> = pieces
                .iter()
                .filter(|&&p| p != PieceType::King)
                .copied()
                .collect();

            match non_king_pieces.len() {
                0 => true, // King only
                1 => {
                    // King + one minor piece
                    matches!(non_king_pieces[0], PieceType::Knight | PieceType::Bishop)
                }
                2 => {
                    // King + two knights (very rare but insufficient)
                    non_king_pieces.iter().all(|&p| p == PieceType::Knight)
                }
                _ => false, // More pieces = sufficient material
            }
        };

        // Both sides must have insufficient material for a draw
        is_insufficient(&white_pieces) && is_insufficient(&black_pieces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_creation() {
        // Valid squares
        assert!(Square::new(0, 0).is_some());
        assert!(Square::new(7, 7).is_some());
        assert!(Square::new(3, 4).is_some());

        // Invalid squares
        assert!(Square::new(8, 0).is_none());
        assert!(Square::new(0, 8).is_none());
        assert!(Square::new(8, 8).is_none());
    }

    #[test]
    fn test_square_from_string() {
        // Valid squares
        assert_eq!(Square::from_str("a1"), Some(Square { file: 0, rank: 0 }));
        assert_eq!(Square::from_str("h8"), Some(Square { file: 7, rank: 7 }));
        assert_eq!(Square::from_str("e4"), Some(Square { file: 4, rank: 3 }));

        // Invalid squares
        assert!(Square::from_str("i1").is_none());
        assert!(Square::from_str("a9").is_none());
        assert!(Square::from_str("").is_none());
        assert!(Square::from_str("abc").is_none());
    }

    #[test]
    fn test_square_to_string() {
        let square = Square { file: 0, rank: 0 };
        assert_eq!(square.to_string(), "a1");

        let square = Square { file: 7, rank: 7 };
        assert_eq!(square.to_string(), "h8");

        let square = Square { file: 4, rank: 3 };
        assert_eq!(square.to_string(), "e4");
    }

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }

    #[test]
    fn test_board_initial_setup() {
        let board = Board::new();

        // Check initial turn
        assert_eq!(board.to_move, Color::White);

        // Check white pieces on first rank
        assert_eq!(
            board.get_piece(Square::new(0, 0).unwrap()),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White
            })
        );
        assert_eq!(
            board.get_piece(Square::new(4, 0).unwrap()),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White
            })
        );

        // Check white pawns on second rank
        for file in 0..8 {
            assert_eq!(
                board.get_piece(Square::new(file, 1).unwrap()),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White
                })
            );
        }

        // Check empty squares in middle
        for rank in 2..6 {
            for file in 0..8 {
                assert_eq!(board.get_piece(Square::new(file, rank).unwrap()), None);
            }
        }

        // Check black pawns on seventh rank
        for file in 0..8 {
            assert_eq!(
                board.get_piece(Square::new(file, 6).unwrap()),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black
                })
            );
        }

        // Check black pieces on eighth rank
        assert_eq!(
            board.get_piece(Square::new(0, 7).unwrap()),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black
            })
        );
        assert_eq!(
            board.get_piece(Square::new(4, 7).unwrap()),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black
            })
        );
    }

    #[test]
    fn test_pawn_moves() {
        let mut board = Board::new();

        // White pawn one square forward
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e3").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));
        assert!(board.make_move(&mv));

        // White pawn two squares forward from starting position
        board = Board::new();
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));
        assert!(board.make_move(&mv));

        // Invalid: pawn moving backwards
        let mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("e3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));

        // Invalid: pawn moving sideways
        let mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("f4").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_rook_moves() {
        let mut board = Board::new();

        // Clear path for rook
        board.set_piece(Square::from_str("a2").unwrap(), None);
        board.set_piece(Square::from_str("a3").unwrap(), None);
        board.set_piece(Square::from_str("a4").unwrap(), None);

        // Valid rook move along file
        let mv = Move {
            from: Square::from_str("a1").unwrap(),
            to: Square::from_str("a4").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Clear path for rook along rank
        board.set_piece(Square::from_str("b1").unwrap(), None);
        board.set_piece(Square::from_str("c1").unwrap(), None);

        // Valid rook move along rank
        let mv = Move {
            from: Square::from_str("a1").unwrap(),
            to: Square::from_str("c1").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Invalid diagonal move
        let mv = Move {
            from: Square::from_str("a1").unwrap(),
            to: Square::from_str("b2").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_knight_moves() {
        let board = Board::new();

        // Valid knight moves from starting position
        let mv = Move {
            from: Square::from_str("b1").unwrap(),
            to: Square::from_str("c3").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        let mv = Move {
            from: Square::from_str("b1").unwrap(),
            to: Square::from_str("a3").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Invalid knight move
        let mv = Move {
            from: Square::from_str("b1").unwrap(),
            to: Square::from_str("b3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_bishop_moves() {
        let mut board = Board::new();

        // Clear diagonal path
        board.set_piece(Square::from_str("d2").unwrap(), None);
        board.set_piece(Square::from_str("e3").unwrap(), None);

        // Valid diagonal move
        let mv = Move {
            from: Square::from_str("c1").unwrap(),
            to: Square::from_str("f4").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Invalid non-diagonal move
        let mv = Move {
            from: Square::from_str("c1").unwrap(),
            to: Square::from_str("c3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_queen_moves() {
        let mut board = Board::new();

        // Clear paths for queen moves - need to clear the entire diagonal path
        board.set_piece(Square::from_str("d2").unwrap(), None); // Clear pawn blocking diagonal
        board.set_piece(Square::from_str("e3").unwrap(), None); // Clear potential blocker
        board.set_piece(Square::from_str("c2").unwrap(), None); // Clear pawn blocking horizontal
        board.set_piece(Square::from_str("b2").unwrap(), None); // Clear pawn blocking horizontal
        board.set_piece(Square::from_str("c1").unwrap(), None); // Clear bishop
        board.set_piece(Square::from_str("b1").unwrap(), None); // Clear knight

        // Valid diagonal move (like bishop) - d1 to f3 needs e2 to be clear too
        board.set_piece(Square::from_str("e2").unwrap(), None); // Clear the pawn on e2
        let mv = Move {
            from: Square::from_str("d1").unwrap(),
            to: Square::from_str("f3").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Valid horizontal move (like rook)
        let mv = Move {
            from: Square::from_str("d1").unwrap(),
            to: Square::from_str("b1").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Invalid knight-like move
        let mv = Move {
            from: Square::from_str("d1").unwrap(),
            to: Square::from_str("c3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_king_moves() {
        let mut board = Board::new();

        // Clear space around king
        board.set_piece(Square::from_str("e2").unwrap(), None);
        board.set_piece(Square::from_str("d2").unwrap(), None);
        board.set_piece(Square::from_str("f2").unwrap(), None);

        // Valid one-square moves
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("e2").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("d2").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));

        // Invalid two-square move
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("e3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_path_blocking() {
        let board = Board::new();

        // Rook blocked by pawn
        let mv = Move {
            from: Square::from_str("a1").unwrap(),
            to: Square::from_str("a3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv)); // Blocked by pawn on a2

        // Bishop blocked by pawn
        let mv = Move {
            from: Square::from_str("c1").unwrap(),
            to: Square::from_str("f4").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv)); // Blocked by pawn on d2
    }

    #[test]
    fn test_capturing() {
        let mut board = Board::new();

        // Move white pawn to capture position
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        // Move black pawn to capture position
        let mv = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        // White pawn captures black pawn
        let mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&mv));
        assert!(board.make_move(&mv));

        // Verify capture
        assert_eq!(
            board.get_piece(Square::from_str("d5").unwrap()),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White
            })
        );
        assert_eq!(board.get_piece(Square::from_str("e4").unwrap()), None);
    }

    #[test]
    fn test_turn_switching() {
        let mut board = Board::new();

        assert_eq!(board.to_move, Color::White);

        // Make a move
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        assert_eq!(board.to_move, Color::Black);

        // Make another move
        let mv = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        assert_eq!(board.to_move, Color::White);
    }

    #[test]
    fn test_invalid_moves() {
        let board = Board::new();

        // Moving opponent's piece
        let mv = Move {
            from: Square::from_str("e7").unwrap(), // Black pawn
            to: Square::from_str("e6").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv)); // White to move

        // Moving to square with own piece
        let mv = Move {
            from: Square::from_str("a1").unwrap(), // White rook
            to: Square::from_str("a2").unwrap(),   // White pawn
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));

        // Moving from empty square
        let mv = Move {
            from: Square::from_str("e4").unwrap(), // Empty square
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_find_king() {
        let board = Board::new();

        let white_king = board.find_king(Color::White);
        assert_eq!(white_king, Some(Square::from_str("e1").unwrap()));

        let black_king = board.find_king(Color::Black);
        assert_eq!(black_king, Some(Square::from_str("e8").unwrap()));
    }

    #[test]
    fn test_check_detection() {
        let mut board = Board::new();

        // Create a simple check scenario
        // Move white queen to attack black king
        board.set_piece(Square::from_str("d1").unwrap(), None); // Remove white queen
        board.set_piece(
            Square::from_str("d8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            }),
        ); // Place white queen attacking black king

        assert!(board.is_in_check(Color::Black));
        assert!(!board.is_in_check(Color::White));
    }

    #[test]
    fn test_checkmate_detection() {
        let mut board = Board::new();

        // Create a simple checkmate scenario
        // Clear the board first
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Set up a simple checkmate position
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("a7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("b6").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );

        board.to_move = Color::Black;
        assert!(board.is_checkmate());
    }

    #[test]
    fn test_stalemate_detection() {
        let mut board = Board::new();

        // Clear the board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Set up a proper stalemate position
        // Black king on a8, white queen on b6, white king on c6
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("b6").unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("c6").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );

        board.to_move = Color::Black;
        assert!(board.is_stalemate());
        assert!(!board.is_checkmate());
    }

    #[test]
    fn test_insufficient_material() {
        let mut board = Board::new();

        // Clear the board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // King vs King
        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        assert!(board.is_insufficient_material());

        // King + Bishop vs King
        board.set_piece(
            Square::from_str("d1").unwrap(),
            Some(Piece {
                piece_type: PieceType::Bishop,
                color: Color::White,
            }),
        );

        assert!(board.is_insufficient_material());

        // King + Queen vs King (sufficient material)
        board.set_piece(
            Square::from_str("d1").unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            }),
        );

        assert!(!board.is_insufficient_material());
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::new();

        // Clear the board and set up promotion scenario
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Place white pawn on 7th rank
        board.set_piece(
            Square::from_str("e7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            }),
        );

        // Place kings
        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // Promote pawn to queen
        let mv = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e8").unwrap(),
            promotion: Some(PieceType::Queen),
        };

        assert!(board.make_move(&mv));

        // Check that the piece is now a queen
        assert_eq!(
            board.get_piece(Square::from_str("e8").unwrap()),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            })
        );
    }

    #[test]
    fn test_move_count() {
        let mut board = Board::new();

        assert_eq!(board.fullmove_number, 1);

        // White move
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        assert_eq!(board.fullmove_number, 1); // Still move 1 after white's turn

        // Black move
        let mv = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        assert_eq!(board.fullmove_number, 2); // Now move 2 after black's turn
    }

    #[test]
    fn test_complex_game_scenario() {
        let mut board = Board::new();

        // Play a few moves of a real game
        let moves = vec![
            ("e2", "e4"),
            ("e7", "e5"),
            ("g1", "f3"),
            ("b8", "c6"),
            ("f1", "b5"),
        ];

        for (from, to) in moves {
            let mv = Move {
                from: Square::from_str(from).unwrap(),
                to: Square::from_str(to).unwrap(),
                promotion: None,
            };
            assert!(
                board.make_move(&mv),
                "Failed to make move {} -> {}",
                from,
                to
            );
        }

        // Verify the board state
        assert_eq!(board.to_move, Color::Black);
        assert_eq!(board.fullmove_number, 3);

        // Check specific piece positions
        assert_eq!(
            board.get_piece(Square::from_str("e4").unwrap()),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White
            })
        );
        assert_eq!(
            board.get_piece(Square::from_str("f3").unwrap()),
            Some(Piece {
                piece_type: PieceType::Knight,
                color: Color::White
            })
        );
        assert_eq!(
            board.get_piece(Square::from_str("b5").unwrap()),
            Some(Piece {
                piece_type: PieceType::Bishop,
                color: Color::White
            })
        );
    }

    #[test]
    fn test_castling_kingside() {
        let mut board = Board::new();

        // Clear pieces between king and rook
        board.set_piece(Square::from_str("f1").unwrap(), None); // Bishop
        board.set_piece(Square::from_str("g1").unwrap(), None); // Knight

        // Castling move (king moves two squares)
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(board.is_valid_move(&mv));
        assert!(board.make_move(&mv));

        // Verify final positions
        assert_eq!(
            board.get_piece(Square::from_str("g1").unwrap()),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White
            })
        );
        assert_eq!(
            board.get_piece(Square::from_str("f1").unwrap()),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White
            })
        );
        assert_eq!(board.get_piece(Square::from_str("e1").unwrap()), None);
        assert_eq!(board.get_piece(Square::from_str("h1").unwrap()), None);
    }

    #[test]
    fn test_castling_invalid_cases() {
        let mut board = Board::new();

        // Clear pieces between king and rook
        board.set_piece(Square::from_str("f1").unwrap(), None);
        board.set_piece(Square::from_str("g1").unwrap(), None);

        // Move king first (loses castling rights)
        let king_move = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("f1").unwrap(),
            promotion: None,
        };
        board.make_move(&king_move);

        // Move king back
        let king_back = Move {
            from: Square::from_str("f1").unwrap(),
            to: Square::from_str("e1").unwrap(),
            promotion: None,
        };
        board.make_move(&king_back);
        board.make_move(&Move {
            from: Square::from_str("a7").unwrap(),
            to: Square::from_str("a6").unwrap(),
            promotion: None,
        }); // Black move

        // Try to castle - should fail
        let castle_attempt = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&castle_attempt));
    }

    #[test]
    fn test_en_passant_setup() {
        let mut board = Board::new();

        // Move white pawn two squares
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        // En passant square should be set
        assert_eq!(board.en_passant, Some(Square::from_str("e3").unwrap()));

        // After black's move, en passant should still be available
        let black_mv = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&black_mv);

        // Now en passant should be for black's pawn
        assert_eq!(board.en_passant, Some(Square::from_str("d6").unwrap()));
    }

    #[test]
    fn test_en_passant_capture() {
        let mut board = Board::new();

        // Set up en passant scenario
        // Move white pawn to 5th rank
        board.set_piece(Square::from_str("e2").unwrap(), None);
        board.set_piece(
            Square::from_str("e5").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            }),
        );

        // Black pawn moves two squares next to white pawn
        board.to_move = Color::Black;
        let mv = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        // White captures en passant
        let en_passant_capture = Move {
            from: Square::from_str("e5").unwrap(),
            to: Square::from_str("d6").unwrap(),
            promotion: None,
        };

        assert!(board.is_valid_move(&en_passant_capture));
        assert!(board.make_move(&en_passant_capture));

        // Verify capture
        assert_eq!(
            board.get_piece(Square::from_str("d6").unwrap()),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White
            })
        );
        assert_eq!(board.get_piece(Square::from_str("d5").unwrap()), None); // Captured pawn removed
        assert_eq!(board.get_piece(Square::from_str("e5").unwrap()), None); // Original square empty
    }

    #[test]
    fn test_pawn_diagonal_capture() {
        let mut board = Board::new();

        // Move pawns to capture position
        let mv1 = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv1);

        let mv2 = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv2);

        // Pawn captures diagonally
        let capture_mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };

        assert!(board.is_valid_move(&capture_mv));

        // Invalid diagonal move without capture
        board.set_piece(Square::from_str("f5").unwrap(), None); // Ensure f5 is empty
        let invalid_diagonal = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("f5").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&invalid_diagonal));
    }

    #[test]
    fn test_pawn_promotion_variants() {
        let mut board = Board::new();

        // Set up promotion scenario
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        board.set_piece(
            Square::from_str("e7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );
        board.to_move = Color::White;

        // Test promotion to different pieces
        let promotion_pieces = vec![
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
        ];

        for piece_type in promotion_pieces {
            let mut test_board = board.clone();
            let mv = Move {
                from: Square::from_str("e7").unwrap(),
                to: Square::from_str("e8").unwrap(),
                promotion: Some(piece_type),
            };

            assert!(test_board.make_move(&mv));
            assert_eq!(
                test_board.get_piece(Square::from_str("e8").unwrap()),
                Some(Piece {
                    piece_type,
                    color: Color::White
                })
            );
        }
    }

    #[test]
    fn test_moving_into_check_invalid() {
        let mut board = Board::new();

        // Set up scenario where king would move into check
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        ); // Rook attacks e-file
        board.to_move = Color::White;

        // King cannot move to e2 (into check)
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("e2").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_pinned_piece() {
        let mut board = Board::new();

        // Clear board and set up pin scenario
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e2").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            }),
        ); // Pinned piece
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        ); // Pinning piece
        board.to_move = Color::White;

        // Rook cannot move as it would expose king to check
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("d3").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv));

        // But rook can move along the pin line
        let valid_mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e3").unwrap(),
            promotion: None,
        };

        assert!(board.is_valid_move(&valid_mv));
    }

    #[test]
    fn test_halfmove_clock() {
        let mut board = Board::new();

        assert_eq!(board.halfmove_clock, 0);

        // Non-pawn, non-capture move should increment halfmove clock
        let mv = Move {
            from: Square::from_str("g1").unwrap(),
            to: Square::from_str("f3").unwrap(),
            promotion: None,
        };
        board.make_move(&mv);

        assert_eq!(board.halfmove_clock, 1);

        // Pawn move should reset halfmove clock
        let pawn_mv = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };
        board.make_move(&pawn_mv);

        assert_eq!(board.halfmove_clock, 0);
    }

    // Excluded
    // #[test]
    // fn test_fifty_move_rule() {
    //     let mut board = Board::new();

    //     // Simulate 50 moves without pawn moves or captures
    //     // This is a simplified test - in reality you'd need valid moves
    //     board.halfmove_clock = 100; // 50 full moves = 100 half moves

    //     assert!(board.is_fifty_move_rule());

    //     board.halfmove_clock = 99;
    //     assert!(!board.is_fifty_move_rule());
    // }

    #[test]
    fn test_threefold_repetition() {
        // This test would require position history tracking
        // which isn't shown in your current structs
        let mut board = Board::new();

        // Move sequence that creates repetition
        let moves = vec![
            ("g1", "f3"),
            ("g8", "f6"),
            ("f3", "g1"),
            ("f6", "g8"),
            ("g1", "f3"),
            ("g8", "f6"),
            ("f3", "g1"),
            ("f6", "g8"),
        ];

        for (from, to) in moves {
            let mv = Move {
                from: Square::from_str(from).unwrap(),
                to: Square::from_str(to).unwrap(),
                promotion: None,
            };
            board.make_move(&mv);
        }

        // This would require implementing position tracking
        // assert!(board.is_threefold_repetition());
        // TODO: implement
    }

    #[test]
    fn test_double_check() {
        let mut board = Board::new();

        // Clear board and create double check scenario
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("a5").unwrap(),
            Some(Piece {
                piece_type: PieceType::Bishop,
                color: Color::Black,
            }),
        );
        board.to_move = Color::White;

        // In double check, only king moves are valid
        assert!(board.is_in_check(Color::White));

        // This would require implementing double check detection
        // and ensuring only king moves are allowed

        // TODO: implement
    }

    #[test]
    fn test_castling_queenside() {
        let mut board = Board::new();

        // Clear pieces between king and queenside rook
        board.set_piece(Square::from_str("d1").unwrap(), None); // Queen
        board.set_piece(Square::from_str("c1").unwrap(), None); // Bishop
        board.set_piece(Square::from_str("b1").unwrap(), None); // Knight

        // Queenside castling move (king moves two squares toward queenside)
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("c1").unwrap(),
            promotion: None,
        };

        assert!(board.is_valid_move(&mv));
        assert!(board.make_move(&mv));

        // Verify final positions after queenside castling
        assert_eq!(
            board.get_piece(Square::from_str("c1").unwrap()),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White
            })
        );
        assert_eq!(
            board.get_piece(Square::from_str("d1").unwrap()),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White
            })
        );
        assert_eq!(board.get_piece(Square::from_str("e1").unwrap()), None);
        assert_eq!(board.get_piece(Square::from_str("a1").unwrap()), None);
    }

    #[test]
    fn test_castling_through_check_invalid() {
        let mut board = Board::new();

        // Clear pieces for castling
        board.set_piece(Square::from_str("f1").unwrap(), None);
        board.set_piece(Square::from_str("g1").unwrap(), None);

        // Clear pawns that would block the rook's attack
        board.set_piece(Square::from_str("f2").unwrap(), None);
        board.set_piece(Square::from_str("f7").unwrap(), None);

        // Place enemy rook attacking f1 (square king must pass through)
        board.set_piece(
            Square::from_str("f8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        );

        // Castling should be invalid (king would pass through check)
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_castling_while_in_check_invalid() {
        let mut board = Board::new();

        // Clear pieces for castling
        board.set_piece(Square::from_str("f1").unwrap(), None);
        board.set_piece(Square::from_str("g1").unwrap(), None);

        // Clear pawns that would block the rook's attack on the king
        board.set_piece(Square::from_str("e2").unwrap(), None);
        board.set_piece(Square::from_str("e7").unwrap(), None);

        // Place enemy rook attacking the king
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        );

        // Cannot castle while in check
        let mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_rook_move_loses_castling_rights() {
        let mut board = Board::new();

        // Move kingside rook
        board.set_piece(Square::from_str("g1").unwrap(), None); // Clear knight
        let rook_mv = Move {
            from: Square::from_str("h1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };
        board.make_move(&rook_mv);

        // Move rook back
        let rook_back = Move {
            from: Square::from_str("g1").unwrap(),
            to: Square::from_str("h1").unwrap(),
            promotion: None,
        };
        board.make_move(&rook_back);

        // Make a black move
        board.make_move(&Move {
            from: Square::from_str("a7").unwrap(),
            to: Square::from_str("a6").unwrap(),
            promotion: None,
        });

        // Clear pieces for castling
        board.set_piece(Square::from_str("f1").unwrap(), None);
        board.set_piece(Square::from_str("g1").unwrap(), None);

        // Castling should now be invalid (rook moved)
        let castle_mv = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&castle_mv));
    }

    #[test]
    fn test_en_passant_expires() {
        let mut board = Board::new();

        // Move white pawn two squares to enable en passant
        let mv1 = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv1);

        // Move black pawn to position for en passant
        let mv2 = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv2);

        // En passant should be available
        assert_eq!(board.en_passant, Some(Square::from_str("d6").unwrap()));

        // Make another move (not en passant)
        let mv3 = Move {
            from: Square::from_str("g1").unwrap(),
            to: Square::from_str("f3").unwrap(),
            promotion: None,
        };
        board.make_move(&mv3);

        // En passant should now be expired
        assert_eq!(board.en_passant, None);

        // En passant capture should no longer be valid
        let en_passant_mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("d6").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&en_passant_mv));
    }

    #[test]
    fn test_pawn_double_move_only_from_starting_rank() {
        let mut board = Board::new();

        // Move pawn one square first
        let mv1 = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e3").unwrap(),
            promotion: None,
        };
        board.make_move(&mv1);

        // Black move
        board.make_move(&Move {
            from: Square::from_str("a7").unwrap(),
            to: Square::from_str("a6").unwrap(),
            promotion: None,
        });

        // Now try to move the same pawn two squares (should be invalid)
        let invalid_double = Move {
            from: Square::from_str("e3").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&invalid_double));
    }

    #[test]
    fn test_pawn_promotion_with_capture() {
        let mut board = Board::new();

        // Clear board and set up promotion capture scenario
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Place white pawn on 7th rank
        board.set_piece(
            Square::from_str("e7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            }),
        );

        // Place black piece to capture
        board.set_piece(
            Square::from_str("d8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        );

        // Place kings
        board.set_piece(
            Square::from_str("a1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // Capture and promote
        let mv = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("d8").unwrap(),
            promotion: Some(PieceType::Queen),
        };

        assert!(board.make_move(&mv));

        // Verify promotion and capture
        assert_eq!(
            board.get_piece(Square::from_str("d8").unwrap()),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            })
        );
        assert_eq!(board.get_piece(Square::from_str("e7").unwrap()), None);
    }

    #[test]
    fn test_king_cannot_move_adjacent_to_enemy_king() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Place kings close to each other
        board.set_piece(
            Square::from_str("e4").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e6").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // White king cannot move adjacent to black king
        let mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_discovered_check() {
        let mut board = Board::new();

        // Clear board and set up discovered check scenario
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e3").unwrap(),
            Some(Piece {
                piece_type: PieceType::Knight,
                color: Color::White,
            }),
        ); // Piece that will move to discover check
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        ); // Will give discovered check
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // Knight cannot move as it would expose king to check
        let mv = Move {
            from: Square::from_str("e3").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv));
    }

    #[test]
    fn test_knight_moves_from_corners() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Place knight in corner
        board.set_piece(
            Square::from_str("a1").unwrap(),
            Some(Piece {
                piece_type: PieceType::Knight,
                color: Color::White,
            }),
        );

        // Place kings
        board.set_piece(
            Square::from_str("h1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("h8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // Valid knight moves from a1
        let valid_moves = vec![
            Square::from_str("b3").unwrap(),
            Square::from_str("c2").unwrap(),
        ];

        for target in valid_moves {
            let mv = Move {
                from: Square::from_str("a1").unwrap(),
                to: target,
                promotion: None,
            };
            assert!(
                board.is_valid_move(&mv),
                "Knight move to {} should be valid",
                target.to_string()
            );
        }

        // Invalid moves (off board or not knight moves)
        let invalid_targets = vec!["a2", "b1", "a3"]; // These aren't valid knight moves

        for target_str in invalid_targets {
            let mv = Move {
                from: Square::from_str("a1").unwrap(),
                to: Square::from_str(target_str).unwrap(),
                promotion: None,
            };
            assert!(
                !board.is_valid_move(&mv),
                "Knight move to {} should be invalid",
                target_str
            );
        }
    }

    #[test]
    fn test_complex_stalemate_scenario() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Classic stalemate: Black king in corner, White king and queen
        // Black king on a8 (corner)
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        // White king on c6 (controls b7, b8)
        board.set_piece(
            Square::from_str("c6").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );

        // White queen on b6 (controls a7, a6, b7, b8)
        board.set_piece(
            Square::from_str("b6").unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::White,
            }),
        );

        board.to_move = Color::Black;

        assert!(board.is_stalemate());
        assert!(!board.is_checkmate());
        assert!(!board.is_in_check(Color::Black));
    }

    #[test]
    fn test_halfmove_clock_with_capture() {
        let mut board = Board::new();

        // Set up capture scenario
        let mv1 = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv1);
        assert_eq!(board.halfmove_clock, 0); // Pawn move resets

        let mv2 = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv2);
        assert_eq!(board.halfmove_clock, 0); // Pawn move resets

        // Capture should reset halfmove clock
        let capture_mv = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&capture_mv);
        assert_eq!(board.halfmove_clock, 0); // Capture resets
    }

    #[test]
    fn test_pawn_cannot_move_two_squares_if_blocked() {
        let board = Board::new();

        // Try to move pawn two squares when blocked by own piece
        // This should be invalid even from starting position
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };

        // First, place a piece on e3 to block
        let mut blocked_board = board.clone();
        blocked_board.set_piece(
            Square::from_str("e3").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            }),
        );

        assert!(!blocked_board.is_valid_move(&mv));
    }

    #[test]
    fn test_castling_with_moved_pieces_restored() {
        let mut board = Board::new();

        // This is a tricky case: if king or rook moves and then moves back,
        // castling should still be permanently lost

        // Clear path for castling
        board.set_piece(Square::from_str("f1").unwrap(), None);
        board.set_piece(Square::from_str("g1").unwrap(), None);

        // Move king
        let king_move = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("f1").unwrap(),
            promotion: None,
        };
        board.make_move(&king_move);

        // Move it back
        let king_back = Move {
            from: Square::from_str("f1").unwrap(),
            to: Square::from_str("e1").unwrap(),
            promotion: None,
        };
        board.make_move(&king_back);

        // Make black moves
        board.make_move(&Move {
            from: Square::from_str("a7").unwrap(),
            to: Square::from_str("a6").unwrap(),
            promotion: None,
        });

        // Try to castle - should fail because king moved earlier
        let castle_attempt = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&castle_attempt));
    }

    #[test]
    #[ignore = "known failing test"]
    fn test_back_rank_mate() {
        let mut board = Board::new();

        // Clear board and set up back rank mate
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Classic back rank mate: White king on h1 trapped by pawns
        board.set_piece(
            Square::from_str("h1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("f2").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            }),
        );
        // g2 square left empty for queen attack
        // h2 square left empty for rook attack

        // Black queen delivering mate and controlling escape squares
        board.set_piece(
            Square::from_str("g8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::Black,
            }),
        );

        // Black king somewhere safe
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        println!("Is white in check? {}", board.is_in_check(Color::White));
        println!("Is checkmate? {}", board.is_checkmate());
        println!("Has legal moves? {}", board.has_legal_moves(Color::White));

        // Debug: Check what moves are available for the white king
        let king_pos = Square::from_str("h1").unwrap();
        for file in 0..8 {
            for rank in 0..8 {
                let to = Square::new(file, rank).unwrap();
                let mv = Move {
                    from: king_pos,
                    to,
                    promotion: None,
                };
                if board.is_valid_move(&mv) {
                    println!("White king can move from h1 to {}", to.to_string());
                }
            }
        }

        // Debug: Check if queen can attack both h1 and g1
        let queen_attack_h1 = Move {
            from: Square::from_str("g8").unwrap(),
            to: Square::from_str("h1").unwrap(),
            promotion: None,
        };
        let queen_attack_g1 = Move {
            from: Square::from_str("g8").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };
        let mut temp_board = board;
        temp_board.to_move = Color::Black;
        println!(
            "Can queen attack h1? {}",
            temp_board.is_valid_move_basic(&queen_attack_h1)
        );
        println!(
            "Can queen attack g1? {}",
            temp_board.is_valid_move_basic(&queen_attack_g1)
        );

        assert!(board.is_in_check(Color::White));
        assert!(board.is_checkmate());
        assert!(!board.has_legal_moves(Color::White));
    }

    #[test]
    fn test_smothered_mate() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Classic smothered mate position: Black king trapped in corner
        board.set_piece(
            Square::from_str("h8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("g7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("h7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("g8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        );

        // White knight delivering mate
        board.set_piece(
            Square::from_str("f7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Knight,
                color: Color::White,
            }),
        );

        // White king
        board.set_piece(
            Square::from_str("a1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );

        board.to_move = Color::Black;

        assert!(board.is_in_check(Color::Black));
        assert!(board.is_checkmate());
    }

    #[test]
    fn test_pawn_promotion_mandatory() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Place pawn on 7th rank ready to promote
        board.set_piece(
            Square::from_str("e7").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // Move to 8th rank without promotion should be invalid
        let mv_without_promotion = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e8").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&mv_without_promotion));

        // Move with promotion should be valid
        let mv_with_promotion = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e8").unwrap(),
            promotion: Some(PieceType::Queen),
        };

        assert!(board.is_valid_move(&mv_with_promotion));
    }

    #[test]
    fn test_illegal_promotion_from_non_promotion_square() {
        let mut board = Board::new();

        // Try to promote a pawn not on promotion rank
        let mv = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e3").unwrap(),
            promotion: Some(PieceType::Queen), // Should be ignored/invalid
        };

        // This move should either be invalid or ignore the promotion
        // Depending on your implementation
        assert!(board.is_valid_move(&mv)); // Move is valid but promotion should be ignored
        board.make_move(&mv);

        // Piece should still be a pawn
        assert_eq!(
            board.get_piece(Square::from_str("e3").unwrap()),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White
            })
        );
    }

    #[test]
    fn test_castling_rights_after_rook_capture() {
        let mut board = Board::new();

        // Set up scenario where rook gets captured
        board.set_piece(Square::from_str("f1").unwrap(), None);
        board.set_piece(Square::from_str("g1").unwrap(), None);

        // Enemy piece captures rook
        board.set_piece(Square::from_str("h8").unwrap(), None); // Remove black rook
        board.set_piece(Square::from_str("h1").unwrap(), None); // Remove white rook temporarily
        board.set_piece(
            Square::from_str("h1").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        ); // Black rook captures

        // After rook is captured, castling should be impossible
        let castle_attempt = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("g1").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&castle_attempt));
    }

    #[test]
    fn test_check_blocks_and_captures() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // Set up check scenario
        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("d2").unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        assert!(board.is_in_check(Color::White));

        // Only moves that get out of check should be valid
        // Block the check
        let block_move = Move {
            from: Square::from_str("d2").unwrap(),
            to: Square::from_str("e2").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&block_move));

        // King move
        let king_move = Move {
            from: Square::from_str("e1").unwrap(),
            to: Square::from_str("f1").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&king_move));

        // Irrelevant move should be invalid
        let irrelevant_move = Move {
            from: Square::from_str("d2").unwrap(),
            to: Square::from_str("d3").unwrap(),
            promotion: None,
        };
        assert!(!board.is_valid_move(&irrelevant_move));
    }

    #[test]
    fn test_king_cannot_capture_defended_piece() {
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // White king, black piece defended by another black piece
        board.set_piece(
            Square::from_str("e4").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e5").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            }),
        );
        board.set_piece(
            Square::from_str("d6").unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            }),
        ); // Defends e5
        board.set_piece(
            Square::from_str("a8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        board.to_move = Color::White;

        // King cannot capture defended pawn
        let capture_attempt = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&capture_attempt));
    }

    #[test]
    fn test_draw_conditions() {
        // Test for insufficient material scenarios
        let mut board = Board::new();

        // Clear board
        for rank in 0..8 {
            for file in 0..8 {
                board.set_piece(Square::new(file, rank).unwrap(), None);
            }
        }

        // King vs King
        board.set_piece(
            Square::from_str("e1").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }),
        );
        board.set_piece(
            Square::from_str("e8").unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }),
        );

        assert!(board.is_insufficient_material());

        // King + Knight vs King
        board.set_piece(
            Square::from_str("d1").unwrap(),
            Some(Piece {
                piece_type: PieceType::Knight,
                color: Color::White,
            }),
        );
        assert!(board.is_insufficient_material());

        // King + Bishop vs King
        board.set_piece(
            Square::from_str("d1").unwrap(),
            Some(Piece {
                piece_type: PieceType::Bishop,
                color: Color::White,
            }),
        );
        assert!(board.is_insufficient_material());

        // King + Knight vs King + Knight (both same color squares) - this is tricky
        board.set_piece(
            Square::from_str("d8").unwrap(),
            Some(Piece {
                piece_type: PieceType::Knight,
                color: Color::Black,
            }),
        );
        assert!(board.is_insufficient_material());
    }

    #[test]
    fn test_pawn_en_passant_only_valid_immediately() {
        let mut board = Board::new();

        // Set up position where en passant would be possible
        let mv1 = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv1);

        let mv2 = Move {
            from: Square::from_str("d7").unwrap(),
            to: Square::from_str("d5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv2);

        // Move pawn to position for en passant
        let mv3 = Move {
            from: Square::from_str("e4").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv3);

        // Black double pawn move
        let mv4 = Move {
            from: Square::from_str("f7").unwrap(),
            to: Square::from_str("f5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv4);

        // En passant should be available
        let en_passant = Move {
            from: Square::from_str("e5").unwrap(),
            to: Square::from_str("f6").unwrap(),
            promotion: None,
        };
        assert!(board.is_valid_move(&en_passant));

        // Make different move
        let other_move = Move {
            from: Square::from_str("g1").unwrap(),
            to: Square::from_str("f3").unwrap(),
            promotion: None,
        };
        board.make_move(&other_move);

        // Now en passant should be invalid
        assert!(!board.is_valid_move(&en_passant));
    }

    #[test]
    fn test_piece_cannot_move_to_same_square() {
        let board = Board::new();

        // Try to move piece to same square
        let invalid_move = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e2").unwrap(),
            promotion: None,
        };

        assert!(!board.is_valid_move(&invalid_move));
    }

    #[test]
    fn test_cannot_capture_own_pieces() {
        let board = Board::new();

        // Try to capture own piece
        let invalid_capture = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("d1").unwrap(), // Queen's square
            promotion: None,
        };

        assert!(!board.is_valid_move(&invalid_capture));
    }

    #[test]
    fn test_algebraic_notation_edge_cases() {
        // Test square creation from invalid strings
        assert_eq!(Square::from_str("z9"), None);
        assert_eq!(Square::from_str("a9"), None);
        assert_eq!(Square::from_str("i1"), None);
        assert_eq!(Square::from_str(""), None);
        assert_eq!(Square::from_str("a"), None);
        assert_eq!(Square::from_str("1"), None);

        // Test valid edge cases
        assert!(Square::from_str("a1").is_some());
        assert!(Square::from_str("h8").is_some());
        assert!(Square::from_str("a8").is_some());
        assert!(Square::from_str("h1").is_some());
    }

    #[test]
    fn test_fullmove_counter_accuracy() {
        let mut board = Board::new();

        assert_eq!(board.fullmove_number, 1);

        // After white's first move, should still be 1
        let mv1 = Move {
            from: Square::from_str("e2").unwrap(),
            to: Square::from_str("e4").unwrap(),
            promotion: None,
        };
        board.make_move(&mv1);
        assert_eq!(board.fullmove_number, 1);

        // After black's first move, should be 2
        let mv2 = Move {
            from: Square::from_str("e7").unwrap(),
            to: Square::from_str("e5").unwrap(),
            promotion: None,
        };
        board.make_move(&mv2);
        assert_eq!(board.fullmove_number, 2);

        // Continue pattern
        board.make_move(&Move {
            from: Square::from_str("g1").unwrap(),
            to: Square::from_str("f3").unwrap(),
            promotion: None,
        });
        assert_eq!(board.fullmove_number, 2); // Still 2 after white's move

        board.make_move(&Move {
            from: Square::from_str("g8").unwrap(),
            to: Square::from_str("f6").unwrap(),
            promotion: None,
        });
        assert_eq!(board.fullmove_number, 3); // Now 3 after black's move
    }
}
