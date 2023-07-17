use chess_wasm::chess::*;

#[test]
fn is_checkmate() {
    let checkmates = vec![
        "8/5r2/4K1q1/4p3/3k4/8/8/8 w - - 0 7",
        "4r2r/p6p/1pnN2p1/kQp5/3pPq2/3P4/PPP3PP/R5K1 b - - 0 2",
        "r3k2r/ppp2p1p/2n1p1p1/8/2B2P1q/2NPb1n1/PP4PP/R2Q3K w kq - 0 8",
        "8/6R1/pp1r3p/6p1/P3R1Pk/1P4P1/7K/8 b - - 0 4",
    ];

    let not_checkmates = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "1R6/8/8/8/8/8/7R/k6K b - - 0 1",
    ];

    let mut chess = Chess::new();

    for (i, checkmate) in checkmates.iter().enumerate() {
        chess.load_fen((checkmate).to_string());

        assert!(chess.is_checkmate(), "index: {}", i);
        assert!(!chess.is_draw(), "index: {}", i);

        chess.clear();
    }

    for (i, not_checkmate) in not_checkmates.iter().enumerate() {
        chess.load_fen((not_checkmate).to_string());

        assert!(!chess.is_checkmate(), "index: {}", i);

        chess.clear();
    }
}
