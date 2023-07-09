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
    chess.set(Piece::ROOK, 119);
    chess.set(Piece::PAWN, 101);
    chess.set(Piece::BLACK_KING, 4);
    chess.set(Piece::BLACK_ROOK, 7);

    // chess.set_turn(BLACK);
    chess.move_piece(101, 69);
    // chess.move_piece(6, 7);

    // chess.set_turn(WHITE);
    println!("{:?}", chess.moves(116));

    println!("{:?}", chess);

    chess.set_turn(BLACK);
    println!("{:?}", chess.moves(4));

    println!("{:?}", chess.get_fen());

    // chess.set_turn(BLACK);
    // chess.set(Piece::KING, 5);

    // chess.load_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string());
    // println!("{}", chess.get_fen());
    // println!("{}", chess.convert_index_algebraic_notation(113));
    // println!("{}", chess.convert_algebraic_notation_to_index("e7"));

    // chess.set(Piece::BLACK_BISHOP, 83);
    // chess.set(Piece::BISHOP, 102);
    // chess.set(Piece::ROOK, 103);

    // chess.set(Piece::BLACK_KING, 100);

    // chess.set(Piece::BISHOP, 53);

    // chess.set(Piece::KING, 116);
    // chess.set(Piece::ROOK, 119);
    // // chess.set(Piece::ROOK, 112);

    // chess.set(Piece::BLACK_BISHOP, 67);
    // chess.set(Piece::BLACK_QUEEN, 102);

    // let moves = chess.moves(116);

    // println!("{:?}", chess.is_insufficient_materials());
}
