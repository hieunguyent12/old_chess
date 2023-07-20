use chess_wasm::chess2::{Chess, Color, PieceType as Piece, SquareCoordinate as Square};

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum MoveType {
    Normal = 0,
    EnPassantMove = 1,
    Capture = 2,
    EnPassantCapture = 4,
    CastleKingside = 8,
    CastleQueenside = 16,
    Promotion = 32,
}

fn main() {
    let a = MoveType::Normal;

    println!("{}", a == MoveType::Normal);
}
