use chess_wasm::chess::*;
use chess_wasm::errors::*;

#[test]
fn move_works() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let next_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("e4"), Ok("e4".to_string()));

    assert_eq!(chess.get_fen(), next_fen);
}

#[test]
fn checkmate() {
    let fen = "7k/3R4/3p2Q1/6Q1/2N1N3/8/8/3R3K w - - 0 1".to_string();
    let next_fen = "3R3k/8/3p2Q1/6Q1/2N1N3/8/8/3R3K b - - 1 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("Rd8"), Ok("Rd8#".to_string()));

    assert!(chess.is_checkmate());

    assert_eq!(chess.get_fen(), next_fen);
}

#[test]
fn white_en_passant() {
    let fen = "rnbqkbnr/pp3ppp/2pp4/4pP2/4P3/8/PPPP2PP/RNBQKBNR w KQkq e6 0 1".to_string();
    let next_fen = "rnbqkbnr/pp3ppp/2ppP3/8/4P3/8/PPPP2PP/RNBQKBNR b KQkq - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("fxe6"), Ok("fxe6".to_string()));

    assert_eq!(chess.get_fen(), next_fen);
}

#[test]
fn black_en_passant() {
    let fen = "rnbqkbnr/pppp2pp/8/4p3/4Pp2/2PP4/PP3PPP/RNBQKBNR b KQkq e3 0 1".to_string();
    let next_fen = "rnbqkbnr/pppp2pp/8/4p3/8/2PPp3/PP3PPP/RNBQKBNR w KQkq - 0 2".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("fxe3"), Ok("fxe3".to_string()));

    assert_eq!(chess.get_fen(), next_fen);
}

#[test]
fn pinning_disambiguates_notation() {
    let fen = "r2qkbnr/ppp2ppp/2n5/1B2pQ2/4P3/8/PPP2PPP/RNB1K2R b KQkq - 3 7".to_string();
    let next_fen = "r2qkb1r/ppp1nppp/2n5/1B2pQ2/4P3/8/PPP2PPP/RNB1K2R w KQkq - 4 8".to_string();

    let mut chess = Chess::new();

    chess.load_fen(fen.clone());
    assert_eq!(chess.move_piece("Ne7"), Ok("Ne7".to_string()));
    assert_eq!(chess.get_fen(), next_fen);

    chess.clear();

    chess.load_fen(fen);
    // Nge7 should be the same as Ne7
    assert_eq!(chess.move_piece("Nge7"), Ok("Nge7".to_string()));
    assert_eq!(chess.get_fen(), next_fen);
}

#[test]
fn illegal_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("e5"), Err(MoveError::InvalidPieceToMove));
}

#[test]
fn cannot_promote_if_pawn_not_in_correct_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let next_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("e4=Q"), Err(MoveError::InvalidPromotion));
    assert_eq!(chess.get_fen(), next_fen);
}

#[test]
fn promotion() {
    let fen = "8/1k5P/8/8/8/8/8/1K6 w - - 0 1".to_string();
    let next_fen = "7N/1k6/8/8/8/8/8/1K6 b - - 0 1".to_string();

    let mut chess = Chess::new();
    chess.load_fen(fen);

    assert_eq!(chess.move_piece("h8=N"), Ok("h8=N".to_string()));
    assert!(chess.is_draw());
    assert_eq!(chess.get_fen(), next_fen);
}
