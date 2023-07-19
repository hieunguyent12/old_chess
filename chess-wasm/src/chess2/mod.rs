mod board;
mod castling;
mod constants;
mod errors;
mod history;
mod piece;
mod square;

use board::*;
use castling::*;
use errors::*;
use piece::*;
use square::*;
use std::collections::HashMap;

pub use constants::Color;
pub use piece::PieceType;
pub use square::SquareCoordinate as Square;

struct Kings {}

pub struct Chess {
    pub board: Board,
    // turn: u8,
    // history: Vec<u8>,

    // /// the kings' positions on the board
    // kings: Kings,

    // white_captures: Vec<u8>,
    // black_captures: Vec<u8>,

    // castling_rights: CastlingRights,

    // /// Record each unique positions on the board.
    // /// If any position occurs 3 times at any point in time, the game is declared draw
    // unique_positions: HashMap<String, u8>,

    // half_moves: u8,
    // full_moves: u8,

    // en_passant_sq: Option<String>,

    // // These fields are for testing purposes
    // _testing_captures: u64,
    // _testing_castles: u64,
    // _testing_checks: u64,
    // _testing_promotions: u64,
    // _testing_moves: HashMap<String, u64>,
}

// we need a Square enum

impl Chess {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    pub fn get(&self, idx: Square) -> Result<Option<Piece>, Error> {
        Ok(self.board.get(idx)?)
    }

    pub fn set(
        &mut self,
        sq: SquareCoordinate,
        piece: PieceType,
        color: Color,
    ) -> Result<(Piece, usize), Error> {
        Ok(self.board.set(sq, piece, color)?)
    }
}
