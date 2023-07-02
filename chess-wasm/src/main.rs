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

    chess.set_turn(BLACK);

    chess.set_piece_at(Piece::BLACK_BISHOP, 53);
    chess.set_piece_at(Piece::BLACK_KING, 51);
    chess.set_piece_at(Piece::PAWN, 68);
    // chess.set_piece_at(BISHOP | BLACK, 98);

    let moves = chess.generate_diagonal_sliding_moves(53);

    // chess.set_piece_at(BISHOP, 102);
    // chess.set_piece_at(KING, 112);
    // chess.set_piece_at(PAWN, 97);
    // chess.set_piece_at(BISHOP | BLACK, 82);

    // let moves = chess.generate_diagonal_sliding_moves(102 as u8);

    println!("{:?}", moves);
}
