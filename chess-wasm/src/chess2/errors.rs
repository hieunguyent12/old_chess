use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoveError {
    #[error("Illegal king side castle")]
    IllegalKingSideCastle,

    #[error("Illegal queen side castle")]
    IllegalQueenSideCastle,

    #[error("Illegal capture")]
    IllegalCapture,

    #[error("Unknown PieceType")]
    UnknownPiece,

    #[error("Ambiguous move notation")]
    AmbiguousMoveNotation,

    #[error("Can't find PieceType to move")]
    InvalidPieceToMove,

    #[error("Invalid promotion")]
    InvalidPromotion,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid index {0}")]
    InvalidIndex(usize),

    #[error("Invalid PieceType type")]
    InvalidPieceType,
}
