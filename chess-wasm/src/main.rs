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
    let default = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    chess.load_fen(default);
    // chess.load_fen(
    //     "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
    // );

    // println!("{:?}", chess.perft(4));
    // println!(
    //     "{} {} {}",
    //     chess.captures / 2,
    //     chess.castles / 2,
    //     chess.checks
    // );

    // chess.load_fen(
    //     "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPBBPpP/R3K2R w KQkq - 0 1".to_string(),
    // );

    // println!("{:?}", chess.inner_moves(116));

    // chess.inner_move_piece(116, 118);

    // println!("{:?}", chess.get_fen());
}
