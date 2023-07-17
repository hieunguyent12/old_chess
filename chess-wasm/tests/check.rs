use chess_wasm::chess::*;

#[test]
fn white_in_check() {
    let mut chess = Chess::new();

    chess.load_fen("rnb1kbnr/pppp1ppp/8/8/4Pp1q/2N5/PPPP2PP/R1BQKBNR w KQkq - 2 4".to_string());

    assert!(chess.in_check());
}

#[test]
fn black_in_check() {
    let mut chess = Chess::new();

    chess.load_fen("rnbqkbnr/pppp2pp/5p2/7Q/4Pp2/2N3P1/PPPP3P/R1B1KBNR b KQkq - 2 4".to_string());

    assert!(chess.in_check());
}

#[test]
fn checkmate_is_check() {
    let mut chess = Chess::new();

    chess.load_fen("R3k3/8/4K3/8/8/8/8/8 b - - 0 1".to_string());

    assert!(chess.in_check());
}

#[test]
fn stalemate_is_not_check() {
    let mut chess = Chess::new();

    chess.load_fen("4k3/4P3/4K3/Q7/8/8/8/8 b - - 0 1".to_string());

    assert!(!chess.in_check());
    assert!(chess.is_draw());
}
