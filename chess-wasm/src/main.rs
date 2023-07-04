use chess_wasm::chess::*;

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x} - ", byte))?;
        }
        Ok(())
    }
}

fn main() {
    let mut chess = Chess::new();
    // chess.set_turn(BLACK);
    chess.set(Piece::PAWN, 100);
    chess.set(Piece::BLACK_PAWN, 69);
    chess.move_piece(100, 68);

    let moves = chess.moves(69);

    // 00000101

    println!("{:?}", chess.board);
    println!("{:?}", moves);
    // println!("{}", chess.in_check());
    // let moves = chess.moves(98);

    // chess.set(Piece::PAWN, 86);
    // chess.set(Piece::KING, 84);
    // chess.set(Piece::BLACK_BISHOP, 69);
    // let moves = chess.moves(86);

    // chess.set_turn(BLACK);

    // chess.set(Piece::BLACK_BISHOP, 53);
    // chess.set(Piece::BLACK_KING, 51);
    // chess.set(Piece::PAWN, 68);
    // // chess.set(BISHOP | BLACK, 98);

    // let moves = chess.generate_diagonal_sliding_moves(53);

    // chess.set(BISHOP, 102);
    // chess.set(KING, 112);
    // chess.set(PAWN, 97);
    // chess.set(BISHOP | BLACK, 82);

    // let moves = chess.generate_diagonal_sliding_moves(102 as u8);

    // println!("{:?}", moves);
}
