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
    // let default_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    chess.load_fen("8/1P2k3/8/8/8/8/1p5K/8 w - - 0 1".to_string());

    chess.move_piece("b8=Q");

    println!("{:?}", chess.get_fen());
}
