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

    chess.set(Piece::PAWN, 50);
    chess.set(Piece::BLACK_PAWN, 20);

    chess.move_piece(50, 51);

    chess.set_turn(BLACK);
    chess.move_piece(20, 52);

    chess.set_turn(WHITE);
    chess.move_piece(51, 36);

    // // chess.set_turn(BLACK);

    // chess.set(Piece::MOVED_PAWN, 51);
    // chess.set(Piece::BLACK_PAWN, 52);
    // chess.set(Piece::EMPTY | 5, 36);

    // // chess.move_piece(20, 52);
    // // chess.set_turn(WHITE);

    // chess.move_piece(51, 36);

    println!("{:?}", chess.board);
    println!("{:?}", chess.white_captures);
}
