use chess_wasm::chess::*;
mod utils;
use utils::compare_vec;

#[test]
fn moves_for_pawn() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves = ["e3", "e4"];

    assert_eq!(chess.moves("e2"), moves);
}

// #[test]
// fn invalid_square() {
//     let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

//     let mut chess = Chess::new();
//     chess.load_fen(fen);

//     let moves: Vec<&str> = vec![];

//     assert_eq!(chess.moves("e9"), moves);
// }

#[test]
fn moves_for_pinned_piece() {
    let fen = "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 2 3".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves: [&str; 0] = [];

    assert_eq!(chess.moves("c3"), moves);
}

// TODO: fix this later, include promotions in moves
#[test]
fn moves_for_promotion() {
    let fen = "8/k7/8/8/8/8/7p/K7 b - - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    // TODO be more explicit
    let moves = ["h1"];

    assert_eq!(chess.moves("h2"), moves);
}

#[test]
fn castling() {
    let fen = "r1bq1rk1/1pp2ppp/p1np1n2/2b1p3/2B1P3/2NP1N2/PPPBQPPP/R3K2R w KQ - 0 8".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves = ["Kf1", "Kd1", "O-O", "O-O-O"];

    assert_eq!(chess.moves("e1"), moves);
}

#[test]
fn no_castling() {
    let fen = "r1bq1rk1/1pp2ppp/p1np1n2/2b1p3/2B1P3/2NP1N2/PPPBQPPP/R3K2R w - - 0 8".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves = ["Kf1", "Kd1"];

    assert_eq!(chess.moves("e1"), moves);
}

#[test]
fn king_trapped_cant_move() {
    let fen = "8/7K/8/8/1R6/k7/1R1p4/8 b - - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves: [&str; 0] = [];

    assert_eq!(chess.moves("a3"), moves);
}

#[test]
fn knight_moves() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves = ["Na3".to_string(), "Nc3".to_string()].to_vec();

    assert!(compare_vec(&chess.moves("b1"), &moves));
}

#[test]
fn en_passant() {
    let fen =
        "rnbq1rk1/4bpp1/p2p1n1p/Ppp1p3/2B1P3/2NP1N1P/1PP2PP1/R1BQ1RK1 w - b6 0 10".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves = ["axb6".to_string()].to_vec();

    assert!(compare_vec(&chess.moves("a5"), &moves));
}

#[test]
fn queen() {
    let fen = "5rk1/1p3rp1/p1n1p3/2p1p2p/2PpP1qP/P2P2P1/1P2QP1K/3R1R2 w - - 0 23".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    let moves = [
        "Qd2".to_string(),
        "Qc2".to_string(),
        "Qe1".to_string(),
        "Qe3".to_string(),
        "Qf3".to_string(),
        "Qxg4".to_string(),
    ]
    .to_vec();

    assert!(compare_vec(&chess.moves("e2"), &moves));
}
