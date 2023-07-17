use chess_wasm::chess::*;

#[test]
fn is_insufficient_materials() {
    let insufficient_materials = vec![
        "8/8/8/8/8/8/8/k6K w - - 0 1",
        "8/2N5/8/8/8/8/8/k6K w - - 0 1",
        "8/2b5/8/8/8/8/8/k6K w - - 0 1",
        "8/b7/3B4/8/8/8/8/k6K w - - 0 1",
        "8/b1B1b1B1/1b1B1b1B/8/8/8/8/1k5K w - - 0 1",
    ];

    let not_insufficient_materials = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "8/2p5/8/8/8/8/8/k6K w - - 0 1",
        "5k1K/7B/8/6b1/8/8/8/8 b - - 0 1",
        "7K/5k1N/8/6b1/8/8/8/8 b - - 0 1",
        "7K/5k1N/8/4n3/8/8/8/8 b - - 0 1",
    ];

    let mut chess = Chess::new();

    for (i, x) in insufficient_materials.iter().enumerate() {
        chess.load_fen((x).to_string());

        assert!(chess.is_insufficient_materials(), "index: {}", i);
        assert!(chess.is_draw(), "index: {}", i);

        chess.clear();
    }

    for (i, x) in not_insufficient_materials.iter().enumerate() {
        chess.load_fen((x).to_string());

        assert!(!chess.is_insufficient_materials(), "index: {}", i);

        chess.clear();
    }
}
