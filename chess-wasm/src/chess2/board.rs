use super::constants::*;
use super::errors::*;
use super::piece::*;
use super::square::*;

#[derive(Debug)]
pub struct Board {
    _board: Vec<Square>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            _board: vec![Square { piece: None }; BOARD_SIZE],
        }
    }

    /// Return the PieceType and its associated value on a square. Return an error if the index is out of range
    /// or if the PieceType value is invalid.
    pub fn get(&self, sq: SquareCoordinate) -> Result<Option<Piece>, Error> {
        let idx = self.is_valid(sq.to_index())?;

        let sq = self._board.get(idx);

        if let Some(sq) = sq {
            return Ok(sq.piece);
        }

        return Err(Error::InvalidIndex(idx));
    }

    /// Put a piece at specific index on the board. Return the piece and index if succeed, or an error if not.
    pub fn set(
        &mut self,
        sq: SquareCoordinate,
        piece_type: PieceType,
        color: Color,
    ) -> Result<(Piece, usize), Error> {
        let idx = self.is_valid(sq.to_index())?;

        let piece = Piece { piece_type, color };

        self._board[idx].piece = Some(piece);

        Ok((piece, idx))
    }

    pub fn remove() {}

    /// Check if the index is exists on the board. Return the index if valid, error if not.
    pub fn is_valid(&self, idx: usize) -> Result<usize, Error> {
        if idx & 0x88 == 0 {
            return Ok(idx);
        } else {
            return Err(Error::InvalidIndex(idx));
        }
    }
}
