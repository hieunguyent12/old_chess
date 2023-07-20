use super::{Board, Kings, SquareCoordinate};

#[derive(Clone)]
pub struct Castling {
    pub kingside: bool,
    pub queenside: bool,
}

#[derive(Clone)]
pub struct CastlingRights {
    pub white: Castling,
    pub black: Castling,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            white: Castling {
                kingside: true,
                queenside: true,
            },
            black: Castling {
                kingside: true,
                queenside: true,
            },
        }
    }

    pub fn update(&mut self, kings: &Kings, board: &Board) {
        // TODO check rooks
        if kings.white != Some(SquareCoordinate::E1) {
            self.white.kingside = false;
            self.white.queenside = false;
        }

        if kings.black != Some(SquareCoordinate::E8) {
            self.black.kingside = false;
            self.black.queenside = false;
        }
    }
}
