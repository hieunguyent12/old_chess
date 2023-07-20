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
pub use square::SquareCoordinate;

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

    // half_moves: u8,
    // full_moves: u8,
    en_passant_sq: Option<SquareCoordinate>,
    // // These fields are for testing purposes
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
        }
    }

    pub fn play_move() {}

    fn make_move(&mut self, m: InternalMove) -> ChessResult<()> {
        let history_entry = HistoryEntry {
            player_move: m.clone(),
            turn: self.turn,
            kings: self.kings.clone(),
            castling_rights: self.castling_rights.clone(),
            en_passant_sq: self.en_passant_sq,
        };

        let from = self.get(m.from_sq)?.ok_or(ChessError::InvalidMove(
            m.from_sq.to_index(),
            m.to_sq.to_index(),
        ))?;

        if m.move_type == MoveType::EnPassantCapture {
            if let Some(en_passant_sq) = self.en_passant_sq {
                if from.color == Color::WHITE {
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

        self.set(m.to_sq, from.piece_type, from.color)?;

        if m.move_type == MoveType::Promotion {
            let promotion_piece = m.promotion_piece.ok_or(ChessError::InvalidPromotion)?;
            self.set(m.to_sq, promotion_piece.piece_type, promotion_piece.color)?;
        }

        self.history.push(history_entry);
        self.castling_rights.update(&self.kings, &self.board);
        // self.change_turn();

        Ok(())
    }

    pub fn undo_move(&mut self) -> ChessResult<()> {
        if let Some(old) = self.history.pop() {
            self.turn = old.turn;
            self.en_passant_sq = old.en_passant_sq;
            self.kings = old.kings;
            self.castling_rights = old.castling_rights;

            let m = old.player_move;

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
                self.set(m.to_sq.right()?, PieceType::ROOK, old.turn)?;
            }

            if m.move_type == MoveType::CastleQueenside {
                // put the rooks back
                self.remove(m.to_sq.right()?)?;
                // put the rooks back two square to the left
                self.set(m.to_sq.subtract(2)?, PieceType::ROOK, old.turn)?;
            }

            self.set(m.from_sq, m.from_piece.piece_type, m.from_piece.color)?;
            if let Some(to_piece) = m.to_piece {
                self.set(m.to_sq, to_piece.piece_type, to_piece.color)?;
            }
        }

        Ok(())
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
            chess.make_move(InternalMove {
                move_type: MoveType::Normal,
                from_sq: Square::E8,
                to_sq: Square::E7,
                from_piece: Piece {
                    piece_type: PieceType::KING,
                    color: Color::BLACK
                },
                to_piece: None,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(
            chess.make_move(InternalMove {
                move_type: MoveType::Normal,
                from_sq: Square::E3,
                to_sq: Square::E7,
                from_piece: Piece {
                    piece_type: PieceType::KING,
                    color: Color::BLACK
                },
                to_piece: None,
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
            chess.make_move(InternalMove {
                move_type: MoveType::EnPassantMove,
                from_sq: Square::E2,
                to_sq: Square::E4,
                from_piece: Piece {
                    piece_type: PieceType::PAWN,
                    color: Color::WHITE
                },
                to_piece: None,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.en_passant_sq, Some(Square::E3));

        assert_eq!(
            chess.make_move(InternalMove {
                move_type: MoveType::Normal,
                from_sq: Square::E4,
                to_sq: Square::E5,
                from_piece: Piece {
                    piece_type: PieceType::PAWN,
                    color: Color::WHITE
                },
                to_piece: None,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.en_passant_sq, None);
    }

    #[test]
    fn en_passant_capture() {}

    #[test]
    fn undo() {
        let mut chess = Chess::new();

        // assigning to empty var to ignore warning
        let _ = chess.set(Square::E2, PieceType::PAWN, Color::WHITE);

        assert_eq!(
            chess.make_move(InternalMove {
                move_type: MoveType::EnPassantMove,
                from_sq: Square::E2,
                to_sq: Square::E4,
                from_piece: Piece {
                    piece_type: PieceType::PAWN,
                    color: Color::WHITE
                },
                to_piece: None,
                promotion_piece: None
            }),
            Ok(())
        );

        assert_eq!(chess.en_passant_sq, Some(Square::E3));

        assert_eq!(chess.undo_move(), Ok(()));

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
    fn castle_kingside() {}

    #[test]
    fn castle_queenside() {}
}
