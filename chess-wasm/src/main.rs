use chess_wasm::chess::*;
use regex::Regex;

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

    chess.load_fen("8/1k6/8/8/8/8/5N2/2Q4K w - - 0 4".to_string());

    // chess.move_piece("Nd3");

    // let re = Regex::new(r"^([KQRBN])([a-h]|[1-8])?(x)?([a-h])([1-8])([+#])?$").unwrap();

    // let captures = re.captures("Nxd2").unwrap();
    // println!("{}", captures.len());

    // println!("{}", captures.get(2).map_or("", |m| m.as_str()));

    println!("{:?}", chess.moves("f2"));
    // println!("{:?}", chess.get_fen());

    // // chess.set_turn(BLACK);
    // chess.set(Piece::KING, 116);
    // chess.set(Piece::ROOK, 119);
    // chess.set(Piece::PAWN, 101);
    // chess.set(Piece::BLACK_KING, 4);
    // chess.set(Piece::BLACK_ROOK, 7);

    // // chess.set_turn(BLACK);
    // chess.inner_move_piece(101, 69);

    // // chess.move_piece(6, 7);

    // // chess.set_turn(WHITE);
    // println!("{:?}", chess.inner_moves(116));

    // chess.set_turn(BLACK);
    // println!("{:?}", chess.inner_moves(4));

    // chess.inner_move_piece(4, 5);

    // println!("{:?}", chess);
}
