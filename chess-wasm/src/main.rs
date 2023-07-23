use chess_wasm::chess2::{Chess, Color, Move, PieceType, SquareCoordinate as Square, *};
use std::error::Error;

// #[derive(Clone, PartialEq)]
// #[repr(u8)]
// pub enum MoveType {
//     Normal = 0,
//     EnPassantMove = 1,
//     Capture = 2,
//     EnPassantCapture = 4,
//     CastleKingside = 8,
//     CastleQueenside = 16,
//     Promotion = 32,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let mut chess = Chess::new();

    // chess.load_fen("k7/4P3/8/8/8/8/8/7K w - - 0 1".to_string());
    chess
        .load_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string())?;

    // let _ = chess.make_move(Move {
    //     from: SquareCoordinate::G8,
    //     to: SquareCoordinate::H8,
    //     promotion_piece: None,
    // });

    // chess.change_turn();

    // println!("{:?}", chess.en_passant_sq);

    // println!("{:?}", chess.moves_for_square(SquareCoordinate::B7));

    // chess.make_move(Move {
    //     from: SquareCoordinate::B7,
    //     to: SquareCoordinate::B8,
    //     promotion_piece: Some(Piece {
    //         piece_type: PieceType::QUEEN,
    //         color: Color::WHITE,
    //     }),
    // })?;

    // use std::time::Instant;
    // let now = Instant::now();

    println!("{:?}", chess.perft(2, true, false));
    println!("{}", chess.promos);

    // let mut sum = 0;
    // for (_, val) in chess.moves {
    //     sum += val;
    // }
    // println!("{}", sum);

    // if let Ok(nodes) = chess.perft(1, true) {
    //     println!("");
    //     println!("{}",);
    // } else {
    //     panic!("perft failed")
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
