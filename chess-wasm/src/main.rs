use chess_wasm::chess2::{Chess, Color, PieceType as Piece, Square};

fn main() {
    let mut chess = Chess::new();

    match chess.set(Square::E1, Piece::KING, Color::WHITE) {
        Ok(_) => println!("{:?}", chess.board),
        Err(msg) => println!("{}", msg),
    };
}
