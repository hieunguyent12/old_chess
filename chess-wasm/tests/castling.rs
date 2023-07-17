use chess_wasm::chess::*;
use chess_wasm::errors::*;

#[test]
fn castling_rights_from_fen() {
    let mut chess = Chess::new();

    chess.load_fen("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1".to_string());

    assert_eq!((false, false), chess.get_castling_rights());
}

#[test]
fn castling_rights_after_king_moves() {
    let mut chess = Chess::new();

    chess.load_fen("r3k2r/8/8/8/8/8/8/R3K2R w QqkK - 0 1".to_string());

    chess.move_piece("Ke2");

    assert_eq!(
        (false, false, true, true),
        chess.get_castling_rights_tests()
    );

    chess.move_piece("Kf7");

    assert_eq!(
        (false, false, false, false),
        chess.get_castling_rights_tests()
    );
}

#[test]
fn castling_rights_after_rook_moves() {
    let mut chess = Chess::new();

    chess.load_fen("r3k2r/8/8/8/8/8/8/R3K2R w QqkK - 0 1".to_string());

    chess.move_piece("Rh2");

    assert_eq!((false, true, true, true), chess.get_castling_rights_tests());

    chess.move_piece("Rh2");

    assert_eq!(
        (false, true, false, true),
        chess.get_castling_rights_tests()
    );

    chess.move_piece("Rb1");

    assert_eq!(
        (false, false, false, true),
        chess.get_castling_rights_tests()
    );

    chess.move_piece("Rb8");

    assert_eq!(
        (false, false, false, false),
        chess.get_castling_rights_tests()
    );
}

#[test]
fn cannot_castle_if_checked() {
    let mut chess = Chess::new();

    let fen = "r3k2r/8/1b6/6q1/8/1B5B/8/R3K2R w KQkq - 0 1".to_string();

    chess.load_fen(fen.clone());

    assert_eq!(
        chess.move_piece("O-O"),
        Err(MoveError::IllegalKingSideCastle)
    );

    assert_eq!(
        chess.move_piece("O-O-O"),
        Err(MoveError::IllegalQueenSideCastle)
    );

    assert_eq!(chess.get_fen(), fen);

    chess.set_turn(BLACK);

    assert_eq!(
        chess.move_piece("O-O"),
        Err(MoveError::IllegalKingSideCastle)
    );

    assert_eq!(
        chess.move_piece("O-O-O"),
        Err(MoveError::IllegalQueenSideCastle)
    );

    chess.set_turn(WHITE);
    assert_eq!(chess.get_fen(), fen);
}
