use chess_wasm::chess::*;

#[test]
fn is_three_fold_repetition() {
    let fen = "8/pp3p1k/2p2q1p/3r1P2/5R2/7P/P1P1QP2/7K b - - 2 30".to_string();

    let moves = vec!["Qe5", "Qh5", "Qf6", "Qe2", "Re5", "Qd3", "Rd5", "Qe2"];

    let mut chess = Chess::new();
    chess.load_fen(fen);

    for _move in moves {
        assert!(!chess.is_threefold_repetition());

        assert_eq!(chess.move_piece(_move), Ok(_move.to_string()));
    }

    assert!(chess.is_threefold_repetition());

    assert_eq!(chess.move_piece("a6"), Ok("a6".to_string()));
    assert!(!chess.is_threefold_repetition());
}

#[test]
fn is_three_fold_repetition_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    let moves = vec!["Nf3", "Nf6", "Ng1", "Ng8", "Nf3", "Nf6", "Ng1", "Ng8"];

    let mut chess = Chess::new();
    chess.load_fen(fen);

    for _move in moves {
        assert!(!chess.is_threefold_repetition());

        assert_eq!(chess.move_piece(_move), Ok(_move.to_string()));
    }

    assert!(chess.is_threefold_repetition());

    assert_eq!(chess.move_piece("e4"), Ok("e4".to_string()));
    assert!(!chess.is_threefold_repetition());
}
