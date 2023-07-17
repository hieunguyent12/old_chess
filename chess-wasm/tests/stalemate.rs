use chess_wasm::chess::*;

#[test]
fn is_stalemate() {
    let stalemates = vec![
        "1R6/8/8/8/8/8/7R/k6K b - - 0 1",
        "8/8/5k2/p4p1p/P4K1P/1r6/8/8 w - - 0 2",
    ];

    let not_stalemates = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "R3k3/8/4K3/8/8/8/8/8 b - - 0 1",
    ];

    let mut chess = Chess::new();

    for (i, x) in stalemates.iter().enumerate() {
        chess.load_fen((x).to_string());

        assert!(chess.is_stalemate(), "index: {}", i);
        assert!(chess.is_draw(), "index: {}", i);

        chess.clear();
    }

    for (i, x) in not_stalemates.iter().enumerate() {
        chess.load_fen((x).to_string());

        assert!(!chess.is_stalemate(), "index: {}", i);

        chess.clear();
    }
}
