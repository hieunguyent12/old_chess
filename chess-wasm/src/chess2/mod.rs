mod board;
mod castling;
mod constants;
mod errors;
mod history;
mod piece;
mod play_move;
mod square;
mod utils;

use board::*;
use castling::*;
use errors::*;
use history::*;
use piece::*;
use play_move::*;
use square::*;
use std::collections::HashMap;
use utils::*;

pub use constants::Color;
pub use piece::PieceType;
pub use piece::*;
pub use play_move::*;
pub use square::SquareCoordinate;

use crate::chess::Piece::ROOK;

use self::constants::{
    BISHOP_DELTAS, BLACK_PAWN_DELTAS, COLOR_MASK, KING_DELTAS, KNIGHT_DELTAS, QUEEN_DELTAS,
    ROOK_DELTAS, WHITE_PAWN_DELTAS,
};

#[derive(Clone)]
pub struct Kings {
    white: Option<SquareCoordinate>,
    black: Option<SquareCoordinate>,
}

pub struct Chess {
    pub board: Board,
    turn: Color,

    /// the kings' positions on the board
    kings: Kings,
    castling_rights: CastlingRights,
    history: MoveHistory,
    pub white_captures: Vec<Piece>,
    pub black_captures: Vec<Piece>,

    // /// Record each unique positions on the board.
    // /// If any position occurs 3 times at any point in time, the game is declared draw
    // unique_positions: HashMap<String, u8>,
    half_moves: u8,
    full_moves: u8,
    en_passant_sq: Option<SquareCoordinate>,
    // _testing_captures: u64,
    // _testing_castles: u64,
    // _testing_checks: u64,
    // _testing_promotions: u64,
    // _testing_moves: HashMap<String, u64>,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::WHITE,
            kings: Kings {
                white: None,
                black: None,
            },
            history: MoveHistory::new(),
            castling_rights: CastlingRights::new(),
            en_passant_sq: None,
            white_captures: vec![],
            black_captures: vec![],
            full_moves: 0,
            half_moves: 0,
        }
    }

    pub fn play_move(&self, m: Move) {}

    pub fn make_move(&mut self, m: Move) -> ChessResult<()> {
        let m = self.convert_to_internal_move(m)?;

        let history_entry = HistoryEntry {
            player_move: m.clone(),
            turn: self.turn,
            kings: self.kings.clone(),
            castling_rights: self.castling_rights.clone(),
            en_passant_sq: self.en_passant_sq,
        };

        self.half_moves += 1;

        self.set(m.to_sq, m.from_piece.piece_type, m.from_piece.color)?;
        self.remove(m.from_sq)?;

        if m.move_type == MoveType::Capture {
            if let Some(to_piece) = m.to_piece {
                if to_piece.color == Color::BLACK {
                    self.white_captures.push(to_piece);
                } else {
                    self.black_captures.push(to_piece);
                }
            }
        }

        if m.move_type == MoveType::EnPassantCapture {
            if let Some(en_passant_sq) = self.en_passant_sq {
                if m.from_piece.color == Color::WHITE {
                    self.white_captures.push(Piece {
                        piece_type: PieceType::PAWN,
                        color: Color::BLACK,
                    });
                    // remove the piece below
                    self.remove(en_passant_sq.below()?)?;
                } else {
                    self.black_captures.push(Piece {
                        piece_type: PieceType::PAWN,
                        color: Color::WHITE,
                    });
                    // remove the piece above
                    self.remove(en_passant_sq.above()?)?;
                }
            }
        }

        if m.move_type == MoveType::EnPassantMove {
            if m.from_piece.color == Color::WHITE {
                self.en_passant_sq = Some(m.to_sq.below()?);
            } else {
                self.en_passant_sq = Some(m.to_sq.above()?);
            }
        } else {
            self.en_passant_sq = None;
        }

        if m.move_type == MoveType::CastleKingside {
            self.remove(m.to_sq.right()?)?;
            self.set(m.to_sq.left()?, PieceType::ROOK, m.from_piece.color)?;
        }

        if m.move_type == MoveType::CastleQueenside {
            self.remove(m.to_sq.subtract(2)?)?;
            self.set(m.to_sq.right()?, PieceType::ROOK, m.from_piece.color)?;
        }

        if m.move_type == MoveType::Promotion {
            let promotion_piece = m.promotion_piece.ok_or(ChessError::InvalidPromotion)?;
            self.set(m.to_sq, promotion_piece.piece_type, promotion_piece.color)?;
        }

        self.history.push(history_entry);
        self.castling_rights.update(&self.kings, &self.board)?;

        // reset half moves if it is a pawn move or a piece is captured
        if m.move_type == MoveType::Capture || m.from_piece.piece_type == PieceType::PAWN {
            self.half_moves = 0;
        }

        Ok(())
    }

    pub fn undo_move(&mut self) -> ChessResult<()> {
        if let Some(old) = self.history.pop() {
            self.turn = old.turn;
            self.en_passant_sq = old.en_passant_sq;
            self.kings = old.kings;
            self.castling_rights = old.castling_rights;

            let m = old.player_move;

            // put the piece back to its original square
            self.set(m.from_sq, m.from_piece.piece_type, m.from_piece.color)?;
            self.remove(m.to_sq)?;

            self.half_moves = self.half_moves.saturating_sub(1);

            if old.turn == Color::BLACK {
                self.full_moves = self.full_moves.saturating_sub(1);
            }

            if m.move_type == MoveType::Capture || m.move_type == MoveType::EnPassantCapture {
                if m.from_piece.color == Color::WHITE {
                    self.white_captures.pop();
                } else {
                    self.black_captures.pop();
                }
            }

            if m.move_type == MoveType::EnPassantCapture {
                // put the captured piece back
                if m.from_piece.color == Color::WHITE {
                    self.set(m.to_sq.below()?, PieceType::PAWN, Color::BLACK)?;
                } else {
                    self.set(m.to_sq.above()?, PieceType::PAWN, Color::WHITE)?;
                }
            }

            if m.move_type == MoveType::CastleKingside {
                // put the rooks back
                self.remove(m.to_sq.left()?)?;
                self.set(m.to_sq.right()?, PieceType::ROOK, m.from_piece.color)?;
            }

            if m.move_type == MoveType::CastleQueenside {
                // put the rooks back
                self.remove(m.to_sq.right()?)?;
                // put the rooks back two square to the left
                self.set(m.to_sq.subtract(2)?, PieceType::ROOK, m.from_piece.color)?;
            }

            if m.move_type == MoveType::Capture {
                if let Some(to_piece) = m.to_piece {
                    // put the captured piece back
                    self.set(m.to_sq, to_piece.piece_type, to_piece.color)?;
                }
            }

            // reset half moves if it is a pawn move or a piece is captured
            if m.move_type == MoveType::Capture || m.from_piece.piece_type == PieceType::PAWN {
                self.half_moves = 0;
            }
        }

        Ok(())
    }

    /// Get all legal moves for a specific piece type
    pub fn moves_for_piece_type() {}

    /// Get all legal moves for the piece on the specified square
    pub fn moves_for_square(&mut self, sq: SquareCoordinate) -> ChessResult<Vec<Move>> {
        let piece = self.get(sq)?.ok_or(ChessError::UnknownError(
            "Can't generate moves for empty square".to_string(),
        ))?;

        match piece.piece_type {
            PieceType::KING => self.king_moves(sq),
            PieceType::QUEEN => self.sliding_moves(sq, QUEEN_DELTAS.to_vec()),
            PieceType::ROOK => self.sliding_moves(sq, ROOK_DELTAS.to_vec()),
            PieceType::BISHOP => self.sliding_moves(sq, BISHOP_DELTAS.to_vec()),
            PieceType::KNIGHT => self.knight_moves(sq),
            PieceType::PAWN => self.pawn_moves(sq),
        }
    }

    /// Get all legal moves for the player to move
    pub fn moves() {}

    pub fn in_check(&self) -> ChessResult<bool> {
        if self.turn == Color::WHITE {
            if let Some(king) = self.kings.white {
                return self.is_attacked(king);
            }
        } else {
            if let Some(king) = self.kings.black {
                return self.is_attacked(king);
            }
        }
        Ok(false)
    }

    pub fn is_attacked(&self, from: SquareCoordinate) -> ChessResult<bool> {
        let sliding_attack_deltas = vec![16, -16, 1, -1, 17, 15, -17, -15];
        let knight_attack_deltas = vec![14, 31, 18, 33, -14, -31, -18, -33];

        let from_idx = from.to_index();
        let from_piece = self.get(from)?;

        for delta in sliding_attack_deltas {
            let mut to = from.to_index() as i16 + delta as i16;

            loop {
                if let Ok(_to) = utils::is_valid(to as usize) {
                    let to_sq = (_to as u8).to_coordinate();

                    if let Some(attacker) = self.get(to_sq)? {
                        if let Some(from_piece) = from_piece {
                            if from_piece.color == attacker.color {
                                break;
                            }
                        } else {
                            return Ok(false);
                        }

                        let diff = from_idx as i16 - to + 119;
                        let attack_bits_mask = constants::ATTACKS[diff as usize];

                        if attack_bits_mask != 0 {
                            if attacker.piece_type == PieceType::PAWN {
                                // let with_color =
                                //     attacker.piece_type.to_value() | attacker.color.to_value();

                                if attacker.color.to_value() == attack_bits_mask & COLOR_MASK {
                                    return Ok(true);
                                }
                            } else {
                                if (attacker.piece_type.to_value() & attack_bits_mask) != 0 {
                                    return Ok(true);
                                }
                            }
                        }
                    }

                    to += delta as i16;
                } else {
                    break;
                }
            }
        }

        for delta in knight_attack_deltas {
            let to = from.to_index() as i16 + delta as i16;

            if let Ok(_to) = utils::is_valid(to as usize) {
                let to_sq = (_to as u8).to_coordinate();

                if let Some(attacker) = self.get(to_sq)? {
                    if !self.is_friendly(attacker) && attacker.piece_type == PieceType::KNIGHT {
                        return Ok(true);
                    }
                }
            } else {
                continue;
            }
        }

        Ok(false)
    }

    pub fn change_turn(&mut self) {
        if self.turn == Color::WHITE {
            self.turn = Color::BLACK
        } else {
            self.turn = Color::WHITE
        }
    }

    /// Return the PieceType and its associated value on a square. Return an ChessError if the index is out of range.
    ///
    /// Some(t) means an occupied square, None means the square is empty.
    pub fn get(&self, sq: SquareCoordinate) -> ChessResult<Option<Piece>> {
        Ok(self.board.get(sq)?)
    }

    /// Put a piece at specific index on the board. Return the piece and index if succeed, or an ChessError if not.
    ///
    /// Note: this does not update castling rights
    pub fn set(
        &mut self,
        sq: SquareCoordinate,
        piece: PieceType,
        color: Color,
    ) -> ChessResult<(Piece, usize)> {
        let s = self.board.set(sq, piece, color)?;

        if piece == PieceType::KING {
            self.update_kings_positions(sq);
        }

        Ok(s)
    }

    pub fn remove(&mut self, sq: SquareCoordinate) -> ChessResult<()> {
        Ok(self.board.remove(sq)?)
    }

    pub fn get_fen() {}

    pub fn load_fen() {}

    fn castling_rights_for_turn(&self) -> (bool, bool) {
        if self.turn == Color::WHITE {
            (
                self.castling_rights.white.kingside,
                self.castling_rights.white.queenside,
            )
        } else {
            (
                self.castling_rights.black.kingside,
                self.castling_rights.black.queenside,
            )
        }
    }

    fn pawn_moves(&self, from: SquareCoordinate) -> ChessResult<Vec<Move>> {
        let mut moves: Vec<Move> = vec![];

        let from_piece = self.get(from)?.ok_or(ChessError::UnknownError(
            "can't generate move for empty piece".to_string(),
        ))?;

        let deltas = match from_piece.color {
            Color::BLACK => WHITE_PAWN_DELTAS,
            Color::WHITE => BLACK_PAWN_DELTAS,
        };

        let mut can_move_forward = true;

        for delta in deltas {
            let to = from.to_index() as i16 + delta as i16;
            if let Ok(_to) = utils::is_valid(to as usize) {
                let to_sq = (_to as u8).to_coordinate();

                if delta % 2 != 0 {
                    // normal capture
                    if let Some(attacker) = self.get(to_sq)? {
                        if !self.is_friendly(attacker) {
                            moves.push(Move {
                                from,
                                to: to_sq,
                                promotion_piece: None,
                            });
                        }
                    }
                } else {
                    if self.get(to_sq)?.is_some() {
                        can_move_forward = false;
                    }

                    if !can_move_forward {
                        continue;
                    }

                    let rank = to_sq.rank();
                    // can only do en passant move if rank is 2 for white or 7 for black
                    if to.abs_diff(from.to_index() as i16) == 32 {
                        if from_piece.color == Color::WHITE && rank == 2 {
                            moves.push(Move {
                                from,
                                to: to_sq,
                                promotion_piece: None,
                            });
                        }

                        if from_piece.color == Color::BLACK && rank == 7 {
                            moves.push(Move {
                                from,
                                to: to_sq,
                                promotion_piece: None,
                            });
                        }
                    }

                    // promotion
                    if rank == 1 || rank == 8 {
                        let color = from_piece.color;
                        let promotion_pieces = vec![
                            Piece {
                                piece_type: PieceType::BISHOP,
                                color,
                            },
                            Piece {
                                piece_type: PieceType::KNIGHT,
                                color,
                            },
                            Piece {
                                piece_type: PieceType::ROOK,
                                color,
                            },
                            Piece {
                                piece_type: PieceType::QUEEN,
                                color,
                            },
                        ];

                        for piece in promotion_pieces {
                            moves.push(Move {
                                from,
                                to: to_sq,
                                promotion_piece: Some(piece),
                            })
                        }
                    }
                }
            } else {
                continue;
            }
        }

        Ok(moves)
    }

    fn king_moves(&mut self, from: SquareCoordinate) -> ChessResult<Vec<Move>> {
        self.castling_rights.update(&self.kings, &self.board)?;

        let mut moves: Vec<Move> = vec![];

        let deltas = KING_DELTAS.to_vec();
        let (can_castle_kingside, can_castle_queenside) = self.castling_rights_for_turn();
        let from_idx = from.to_index();

        for delta in deltas {
            let to = from.to_index() as i16 + delta as i16;

            if let Ok(_to) = utils::is_valid(to as usize) {
                let to_sq = (_to as u8).to_coordinate();

                let diff = to - from_idx as i16;
                let is_castling = diff == 2 || diff == -2;

                // Kingside castle
                if diff == 2 && !can_castle_kingside {
                    continue;
                }

                // Queenside castle
                if diff == -2 && !can_castle_queenside {
                    continue;
                }

                if is_castling {
                    let mut allow_castle = true;
                    let range = match diff {
                        2 => Ok(0..2),
                        -2 => Ok(0..3),
                        _ => Err(ChessError::UnknownError("Illegal castle".to_string())),
                    }?;

                    // kingside
                    for offset in range {
                        let offset = (offset + 1 + from_idx) as u8;

                        // if a piece is blocking the path, we can't castle
                        if self.get(offset.to_coordinate())?.is_some() {
                            allow_castle = false;
                        }

                        // if king is attacked on the path, we can't castle
                    }

                    if !allow_castle {
                        continue;
                    }
                }

                if let Some(piece) = self.get(to_sq)? {
                    // if we encounter a friendly piece, we can't move there
                    if self.is_friendly(piece) {
                        continue;
                    }
                }

                moves.push(Move {
                    from,
                    to: to_sq,
                    promotion_piece: None,
                });
            } else {
                continue;
            }
        }

        Ok(moves)
    }

    fn knight_moves(&self, from: SquareCoordinate) -> ChessResult<Vec<Move>> {
        let mut moves: Vec<Move> = vec![];

        let deltas = KNIGHT_DELTAS.to_vec();

        for delta in deltas {
            let to = from.to_index() as i16 + delta as i16;
            if let Ok(_to) = utils::is_valid(to as usize) {
                let to_sq = (_to as u8).to_coordinate();

                if let Some(piece) = self.get(to_sq)? {
                    // if we encounter a friendly piece, we can't move there
                    if self.is_friendly(piece) {
                        continue;
                    }
                }

                moves.push(Move {
                    from,
                    to: to_sq,
                    promotion_piece: None,
                });
            } else {
                continue;
            }
        }

        Ok(moves)
    }

    fn sliding_moves(&self, from: SquareCoordinate, deltas: Vec<i8>) -> ChessResult<Vec<Move>> {
        let mut moves: Vec<Move> = vec![];

        for delta in deltas {
            let mut to = from.to_index() as i16 + delta as i16;

            loop {
                if let Ok(_to) = utils::is_valid(to as usize) {
                    let to_sq = (_to as u8).to_coordinate();

                    if let Some(piece) = self.get(to_sq)? {
                        // if we encounter a friendly piece, we can't move there
                        if self.is_friendly(piece) {
                            break;
                        } else {
                            // if we encounter an enemy piece, we can capture it but cannot move further
                            moves.push(Move {
                                from,
                                to: to_sq,
                                promotion_piece: None,
                            });
                            break;
                        }
                    } else {
                        moves.push(Move {
                            from,
                            to: to_sq,
                            promotion_piece: None,
                        });
                    }

                    // if the destination square is on the board, we keep searching in that direction until we go off the board
                    to += delta as i16;
                } else {
                    break;
                }
            }
        }

        Ok(moves)
    }

    fn convert_to_internal_move(&self, m: Move) -> ChessResult<InternalMove> {
        let from_piece = self
            .get(m.from)?
            .ok_or(ChessError::InvalidMove(m.from.to_index(), m.to.to_index()))?;

        let to_piece = self.get(m.to)?;

        // create an Internal Move with some defaults
        let mut internal_move = InternalMove {
            move_type: MoveType::Normal,
            from_sq: m.from,
            from_piece,
            to_sq: m.to,
            to_piece,
            promotion_piece: None,
        };

        // capture
        if let Some(to_piece) = to_piece {
            if !self.is_friendly(to_piece) {
                // if the piece isn't friendly, then it is a capture
                internal_move.move_type = MoveType::Capture;
                internal_move.to_piece = Some(to_piece);
            }
        }

        if from_piece.piece_type == PieceType::PAWN {
            // en passant move
            if m.to.to_index().abs_diff(m.from.to_index()) == 32 {
                internal_move.move_type = MoveType::EnPassantMove;
            }

            // en passant capture
            if let Some(en_passant_sq) = self.en_passant_sq {
                if m.to == en_passant_sq {
                    internal_move.move_type = MoveType::EnPassantCapture;
                }
            }

            let rank = m.to.rank();
            // promotion
            if rank == 8 || rank == 1 {
                internal_move.move_type = MoveType::Promotion;
                internal_move.promotion_piece = m.promotion_piece;
            }
        }

        // castling
        if from_piece.piece_type == PieceType::KING {
            let diff = m.to.to_index() as i8 - m.from.to_index() as i8;
            if diff == 2 {
                internal_move.move_type = MoveType::CastleKingside;
            }

            if diff == -2 {
                internal_move.move_type = MoveType::CastleQueenside;
            }
        }

        Ok(internal_move)
    }

    fn is_friendly(&self, piece: Piece) -> bool {
        self.turn == piece.color
    }

    fn update_kings_positions(&mut self, new_sq: SquareCoordinate) {
        if self.turn == Color::WHITE {
            self.kings.white = Some(new_sq)
        } else {
            self.kings.black = Some(new_sq)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PieceType, SquareCoordinate as Square, *};

    #[test]
    fn board_set_and_get_pieces() {
        let mut chess = Chess::new();

        assert_eq!(
            chess.set(Square::__BAD_COORD, PieceType::KING, Color::WHITE),
            Err(ChessError::InvalidIndex(Square::__BAD_COORD.to_index()))
        );

        assert_eq!(chess.get(Square::E1), Ok(None));

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E1, PieceType::KING, Color::WHITE);

        assert_eq!(
            chess.get(Square::E1),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::WHITE
            }))
        );

        let _ = chess.remove(Square::E1);

        assert_eq!(chess.get(Square::E1), Ok(None));
    }

    #[test]
    fn make_move() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E8, PieceType::KING, Color::BLACK);

        assert_eq!(
            chess.get(Square::E8),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::BLACK
            }))
        );

        assert_eq!(
            chess.make_move(Move {
                from: Square::E8,
                to: Square::E7,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(
            chess.make_move(Move {
                from: Square::E3,
                to: Square::E7,
                promotion_piece: None
            }),
            Err(ChessError::InvalidMove(
                Square::E3.to_index(),
                Square::E7.to_index()
            ))
        );
    }

    #[test]
    fn en_passant_move() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E2, PieceType::PAWN, Color::WHITE);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E2,
                to: Square::E4,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.en_passant_sq, Some(Square::E3));

        assert_eq!(
            chess.make_move(Move {
                from: Square::E4,
                to: Square::E5,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.en_passant_sq, None);
    }

    #[test]
    fn en_passant_capture() {
        let mut chess = Chess::new();

        let _ = chess.set(Square::E5, PieceType::PAWN, Color::WHITE);
        let _ = chess.set(Square::D5, PieceType::PAWN, Color::BLACK);

        chess.en_passant_sq = Some(Square::D6);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E5,
                to: Square::D6,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.get(Square::D5), Ok(None));
        assert_eq!(chess.en_passant_sq, None);
        assert_eq!(
            chess.white_captures,
            vec![Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK
            }]
        )
    }

    #[test]
    fn undo_normal_move() {
        let mut chess = Chess::new();

        let _ = chess.set(Square::E1, PieceType::KING, Color::WHITE);

        let _ = chess.make_move(Move {
            from: Square::E1,
            to: Square::E2,
            promotion_piece: None,
        });

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(chess.get(Square::E2), Ok(None));

        assert_eq!(
            chess.get(Square::E1),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::WHITE,
            }))
        );
    }

    #[test]
    fn undo_en_passant_capture() {
        let mut chess = Chess::new();

        let _ = chess.set(Square::E5, PieceType::PAWN, Color::WHITE);
        let _ = chess.set(Square::D5, PieceType::PAWN, Color::BLACK);

        chess.en_passant_sq = Some(Square::D6);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E5,
                to: Square::D6,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(chess.get(Square::D6), Ok(None));
        assert_eq!(
            chess.get(Square::D5),
            Ok(Some(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK
            }))
        );
        assert_eq!(
            chess.get(Square::E5),
            Ok(Some(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE
            }))
        );
    }

    #[test]
    fn undo_en_passant_move() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E2, PieceType::PAWN, Color::WHITE);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E2,
                to: Square::E4,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.en_passant_sq, Some(Square::E3));

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(chess.get(Square::E4), Ok(None));

        assert_eq!(
            chess.get(Square::E2),
            Ok(Some(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE
            }))
        );

        assert_eq!(chess.en_passant_sq, None);
    }

    #[test]
    fn undo_capture() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::F1, PieceType::BISHOP, Color::WHITE);
        let _ = chess.set(Square::C4, PieceType::QUEEN, Color::BLACK);

        assert_eq!(
            chess.make_move(Move {
                from: Square::F1,
                to: Square::C4,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(
            chess.white_captures,
            vec![Piece {
                piece_type: PieceType::QUEEN,
                color: Color::BLACK
            }]
        );

        assert_eq!(
            chess.get(Square::C4),
            Ok(Some(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::WHITE
            }))
        );

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(chess.white_captures, vec![]);

        assert_eq!(
            chess.get(Square::F1),
            Ok(Some(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::WHITE
            }))
        );

        assert_eq!(
            chess.get(Square::C4),
            Ok(Some(Piece {
                piece_type: PieceType::QUEEN,
                color: Color::BLACK
            }))
        );
    }

    #[test]
    fn undo_kingside_castle() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E1, PieceType::KING, Color::WHITE);
        let _ = chess.set(Square::H1, PieceType::ROOK, Color::WHITE);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E1,
                to: Square::G1,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(
            chess.get(Square::E1),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::WHITE
            }))
        );

        assert_eq!(
            chess.get(Square::H1),
            Ok(Some(Piece {
                piece_type: PieceType::ROOK,
                color: Color::WHITE
            }))
        );

        assert_eq!(chess.get(Square::G1), Ok(None));
        assert_eq!(chess.get(Square::F1), Ok(None));
        assert_eq!(chess.castling_rights.white.kingside, true);
        assert_eq!(chess.castling_rights.white.queenside, true);
    }

    #[test]
    fn undo_queenside_castle() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E8, PieceType::KING, Color::BLACK);
        let _ = chess.set(Square::A8, PieceType::ROOK, Color::BLACK);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E8,
                to: Square::C8,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(
            chess.get(Square::E8),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::BLACK
            }))
        );

        assert_eq!(
            chess.get(Square::A8),
            Ok(Some(Piece {
                piece_type: PieceType::ROOK,
                color: Color::BLACK
            }))
        );

        assert_eq!(chess.get(Square::C8), Ok(None));
        assert_eq!(chess.get(Square::D8), Ok(None));
        assert_eq!(chess.castling_rights.black.kingside, true);
        assert_eq!(chess.castling_rights.black.queenside, true);
    }

    #[test]
    fn undo_kingside_promotion() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E7, PieceType::PAWN, Color::WHITE);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E7,
                to: Square::E8,
                promotion_piece: Some(Piece {
                    piece_type: PieceType::QUEEN,
                    color: Color::WHITE
                })
            }),
            Ok(())
        );

        assert_eq!(chess.undo_move(), Ok(()));

        assert_eq!(
            chess.get(Square::E7),
            Ok(Some(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE
            }))
        );

        assert_eq!(chess.get(Square::E8), Ok(None));
    }

    #[test]
    fn castle_kingside_successfully() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E1, PieceType::KING, Color::WHITE);
        let _ = chess.set(Square::H1, PieceType::ROOK, Color::WHITE);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E1,
                to: Square::G1,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.get(Square::H1), Ok(None));

        assert_eq!(
            chess.get(Square::G1),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::WHITE
            }))
        );

        assert_eq!(
            chess.get(Square::F1),
            Ok(Some(Piece {
                piece_type: PieceType::ROOK,
                color: Color::WHITE
            }))
        );

        assert_eq!(chess.castling_rights.white.kingside, false);
        assert_eq!(chess.castling_rights.white.queenside, false);
    }

    #[test]
    fn castle_queenside_successfully() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E8, PieceType::KING, Color::BLACK);
        let _ = chess.set(Square::A8, PieceType::ROOK, Color::BLACK);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E8,
                to: Square::C8,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.get(Square::A8), Ok(None));

        assert_eq!(
            chess.get(Square::C8),
            Ok(Some(Piece {
                piece_type: PieceType::KING,
                color: Color::BLACK
            }))
        );

        assert_eq!(
            chess.get(Square::D8),
            Ok(Some(Piece {
                piece_type: PieceType::ROOK,
                color: Color::BLACK
            }))
        );

        assert_eq!(chess.castling_rights.black.queenside, false);
        assert_eq!(chess.castling_rights.black.kingside, false);
    }

    #[test]
    fn cannot_castle_if_rook_not_in_correct_square() {
        let mut chess = Chess::new();

        let _ = chess.set(Square::E1, PieceType::KING, Color::WHITE);
        let _ = chess.set(Square::H1, PieceType::ROOK, Color::WHITE);
        let _ = chess.set(Square::A1, PieceType::ROOK, Color::WHITE);

        let _ = chess.make_move(Move {
            from: Square::H1,
            to: Square::H2,
            promotion_piece: None,
        });

        assert_eq!(chess.castling_rights.white.queenside, true);
        assert_eq!(chess.castling_rights.white.kingside, false);

        let _ = chess.make_move(Move {
            from: Square::A1,
            to: Square::D1,
            promotion_piece: None,
        });

        assert_eq!(chess.castling_rights.white.queenside, false);
        assert_eq!(chess.castling_rights.white.kingside, false);
    }

    #[test]
    fn promotion() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E7, PieceType::PAWN, Color::WHITE);

        assert_eq!(
            chess.make_move(Move {
                from: Square::E7,
                to: Square::E8,
                promotion_piece: Some(Piece {
                    piece_type: PieceType::QUEEN,
                    color: Color::WHITE
                })
            }),
            Ok(())
        );

        assert_eq!(
            chess.get(Square::E8),
            Ok(Some(Piece {
                piece_type: PieceType::QUEEN,
                color: Color::WHITE
            }))
        );

        assert_eq!(chess.get(Square::E7), Ok(None))
    }
}
