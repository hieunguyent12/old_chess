use super::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
#[repr(u8)]
pub enum PieceType {
    // EMPTY = 0,
    PAWN = 1,
    KNIGHT = 2,
    BISHOP = 4,
    ROOK = 8,
    QUEEN = 16,
    KING = 32,
}

impl PieceType {
    /// Convert a `PieceType` enum to its associated value (u8)
    pub fn to_value(&self) -> u8 {
        *self as u8
    }

    /// Convert a value to its corresponding PieceType type
    pub fn from_value(value: u8) -> Option<PieceType> {
        match value {
            // 0 => Some(PieceType::EMPTY),
            1 => Some(PieceType::PAWN),
            2 => Some(PieceType::KNIGHT),
            4 => Some(PieceType::BISHOP),
            8 => Some(PieceType::ROOK),
            16 => Some(PieceType::QUEEN),
            32 => Some(PieceType::KING),
            _ => None,
        }
    }

    pub fn from_string() -> PieceType {
        todo!()
    }
}
