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
    let fen =
        "rnbq1rk1/4bpp1/p2p1n1p/Ppp1p3/2B1P3/2NP1N1P/1PP2PP1/R1BQ1RK1 w - b6 0 10".to_string();

    chess.load_fen(fen);

    println!("{:?}", chess.moves("a5"));
}
