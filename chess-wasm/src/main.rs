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
    chess.set(Piece::KING, 116);
    chess.set(Piece::BLACK_BISHOP, 83);
    chess.set(Piece::ROOK, 119);

    // chess.set(Piece::BISHOP, 53);

    // chess.set(Piece::KING, 116);
    // chess.set(Piece::ROOK, 119);
    // // chess.set(Piece::ROOK, 112);

    // chess.set(Piece::BLACK_BISHOP, 67);
    // chess.set(Piece::BLACK_QUEEN, 102);

    let moves = chess.moves(116);

    println!("{:?}", moves);
}
