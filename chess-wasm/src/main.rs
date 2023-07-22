use chess_wasm::chess2::{
    Chess, Color, Move, MoveType, Piece, PieceType, SquareCoordinate as Square,
};

// #[derive(Clone, PartialEq)]
// #[repr(u8)]
// pub enum MoveType {
//     Normal = 0,
//     EnPassantMove = 1,
//     Capture = 2,
//     EnPassantCapture = 4,
//     CastleKingside = 8,
//     CastleQueenside = 16,
//     Promotion = 32,
// }

fn main() {
    let mut chess = Chess::new();

    // assigning to empty var to ignore warning
    // chess.change_turn();
    let _ = chess.set(Square::H1, PieceType::QUEEN, Color::BLACK);

    let a = chess.moves_for_square(Square::H1);

    if let Ok(a) = a {
        for b in a {
            println!("{}", b.to.to_index());
        }
    }
}
