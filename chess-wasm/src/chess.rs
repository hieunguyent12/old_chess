use crate::chess::Piece::*;
use std::{collections::HashMap, hash::Hash};
use wasm_bindgen::prelude::*;

const BOARD_SIZE: u8 = 128;
const COLOR_MASK: u8 = 128; // 10000000
const MOVED_MASK: u8 = 64; // 01000000
const EN_PASSANT_SQUARE: u8 = 5; // 00000101

// use | to make a piece black or white
// use & to check if a piece is black or white
pub const WHITE: u8 = 0;
pub const BLACK: u8 = 128;

#[allow(non_snake_case)]
pub mod Piece {
    use crate::chess::*;
    use wasm_bindgen::prelude::*;

    pub type PieceIndex = u8;
    pub type PieceType = u8;

    pub const EMPTY: PieceType = 0; //         00000000
    pub const MOVED_PAWN: PieceType = PAWN | MOVED_MASK; //    10000001
    pub const MOVED_BLACK_PAWN: PieceType = MOVED_PAWN | BLACK; // 11000001
    pub const MOVED_KING: PieceType = KING | MOVED_MASK;

    pub const PAWN: PieceType = 1; //          00000001
    pub const ROOK: PieceType = 2; //          00000010
    pub const KNIGHT: PieceType = 4; //        00000100
    pub const BISHOP: PieceType = 8; //        00001000
    pub const KING: PieceType = 16; //         00010000
    pub const QUEEN: PieceType = 32; //        00100000

    pub const BLACK_PAWN: PieceType = PAWN | BLACK;
    pub const BLACK_ROOK: PieceType = ROOK | BLACK;
    pub const BLACK_KNIGHT: PieceType = KNIGHT | BLACK;
    pub const BLACK_BISHOP: PieceType = BISHOP | BLACK;
    pub const BLACK_KING: PieceType = KING | BLACK;
    pub const BLACK_QUEEN: PieceType = QUEEN | BLACK;
}

const WHITE_PAWN_DELTAS: [i8; 4] = [-16, -32, -17, -15];
const BLACK_PAWN_DELTAS: [i8; 4] = [16, 32, 17, 15];
const MOVED_WHITE_PAWN_DELTAS: [i8; 4] = [-16, 0, -17, -15];
const MOVED_BLACK_PAWN_DELTAS: [i8; 4] = [16, 0, 17, 15];
const BISHOP_DELTAS: [i8; 4] = [17, 15, -17, -15];
const ROOK_DELTAS: [i8; 4] = [16, -16, 1, -1];
const QUEEN_DELTAS: [i8; 8] = [16, -16, 1, -1, 17, 15, -17, -15];
const KNIGHT_DELTAS: [i8; 8] = [14, 31, 18, 33, -14, -31, -18, -33];
const KING_DELTAS: [i8; 10] = [1, 16, 17, 15, -1, -16, -17, -15, 2, -2];
const MOVED_KING_DELTAS: [i8; 8] = [1, 16, 17, 15, -1, -16, -17, -15];

// why does it have 239 items?
// because of how the indexes of the squares on the real board are laid out due to the fact that
// we have to use some indexes in between to represent the dummy board
/*
    q & b = 40
    q & r = 34
    q & b & k = 56

    queen, bishop, king = 56

    queen, bishop, white pawn, king = 57  = 00111001
    queen, bishop, black pawn, king = 185 = 10111001

    00100000
    00001000
    10000001
    00010000



    queen, bishop can move diagonally to any square = 40 = 00101000

    knight can move in L shape = 4 = 00000100

    queen, rook can move vertically or horizontally to any square = 34 = 00100010

    queen, rook, king = 50 = 00110010

  what is the signifance of the fact that the diff. between each square is unique?
  because they are unique, we can store every single diff. (offset by 119) in an array for lookup.
  This way, we can quickly check if a square can be attacked by just finding the difference between the indexes
*/
#[rustfmt::skip]
const ATTACKS: [u8; 239]= [
   40, 0, 0, 0, 0, 0, 0, 34,  0, 0, 0, 0, 0, 0,40, 0, // Notice how the non-zero numbers are placed very specifically,
   0, 40, 0, 0, 0, 0, 0, 34,  0, 0, 0, 0, 0,40, 0, 0, 
   0, 0, 40, 0, 0, 0, 0, 34,  0, 0, 0, 0,40, 0, 0, 0,
   0, 0, 0, 40, 0, 0, 0, 34,  0, 0, 0,40, 0, 0, 0, 0,
   0, 0, 0, 0, 40, 0, 0, 34,  0, 0,40, 0, 0, 0, 0, 0,
   0, 0, 0, 0, 0, 40, 4, 34,  4,40, 0, 0, 0, 0, 0, 0,
   0, 0, 0, 0, 0, 4, 57, 50, 57, 4, 0, 0, 0, 0, 0, 0,
   34,34,34,34,34,34,50,  0, 50,34,34,34,34,34,34, 0, // Note the zero in the very middle, it basically represents the current piece that is being evaluated for attacks
   0, 0, 0, 0, 0, 4,185, 50,185, 4, 0, 0, 0, 0, 0, 0, // But the piece isn't always in the middle? We can "move" it to the middle by adding 119
   0, 0, 0, 0, 0, 40, 4, 34, 4, 40, 0, 0, 0, 0, 0, 0, // and then applying the difference between two squares to find the index relative to the piece in the middle
   0, 0, 0, 0, 40, 0, 0, 34, 0, 0, 40, 0, 0, 0, 0, 0,
   0, 0, 0, 40, 0, 0, 0, 34, 0, 0, 0, 40, 0, 0, 0, 0,
   0, 0, 40, 0, 0, 0, 0, 34, 0, 0, 0, 0, 40, 0, 0, 0,
   0, 40, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 40, 0, 0,
   40, 0, 0, 0, 0, 0, 0, 34,  0, 0, 0, 0, 0, 0, 40, 
];

#[rustfmt::skip]
const DELTAS: [i8; 239]= [
   -17,  0,  0,  0,  0,  0,  0,-16,  0,  0,  0,  0,  0,  0,-15, 0,
     0,-17,  0,  0,  0,  0,  0,-16,  0,  0,  0,  0,  0,-15,  0, 0,
     0,  0,-17,  0,  0,  0,  0,-16,  0,  0,  0,  0,-15,  0,  0, 0,
     0,  0,  0,-17,  0,  0,  0,-16,  0,  0,  0,-15,  0,  0,  0, 0,
     0,  0,  0,  0,-17,  0,  0,-16,  0,  0,-15,  0,  0,  0,  0, 0,
     0,  0,  0,  0,  0,-17,  0,-16,  0,-15,  0,  0,  0,  0,  0, 0,
     0,  0,  0,  0,  0,  0,-17,-16,-15,  0,  0,  0,  0,  0,  0, 0,
    -1, -1, -1, -1, -1, -1, -1,  0,  1,  1,  1,  1,  1,  1,  1, 0,
     0,  0,  0,  0,  0,  0, 15, 16, 17,  0,  0,  0,  0,  0,  0, 0,
     0,  0,  0,  0,  0, 15,  0, 16,  0, 17,  0,  0,  0,  0,  0, 0,
     0,  0,  0,  0, 15,  0,  0, 16,  0,  0, 17,  0,  0,  0,  0, 0,
     0,  0,  0, 15,  0,  0,  0, 16,  0,  0,  0, 17,  0,  0,  0, 0,
     0,  0, 15,  0,  0,  0,  0, 16,  0,  0,  0,  0, 17,  0,  0, 0,
     0, 15,  0,  0,  0,  0,  0, 16,  0,  0,  0,  0,  0, 17,  0, 0,
    15,  0,  0,  0,  0,  0,  0, 16,  0,  0,  0,  0,  0,  0, 17
 ];

const ACTUAL_BOARD: [u8; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23, 32, 33, 34, 35, 36, 37, 38, 39, 48, 49,
    50, 51, 52, 53, 54, 55, 64, 65, 66, 67, 68, 69, 70, 71, 80, 81, 82, 83, 84, 85, 86, 87, 96, 97,
    98, 99, 100, 101, 102, 103, 112, 113, 114, 115, 116, 117, 118, 119,
];

const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
#[derive(Debug)]
struct Move {
    from_idx: PieceIndex,
    to_idx: PieceIndex,
    from_piece: PieceType,
    to_piece: PieceType,
    castle: bool,
    captured: bool,
    en_passant: bool,
    en_passant_square: Option<PieceIndex>,
}
#[derive(Debug)]
struct King {
    white: PieceIndex,
    black: PieceIndex,
}
#[derive(Debug)]
pub struct Chess {
    pub board: [PieceType; BOARD_SIZE as usize],

    /// 0 = white, 128 = black
    turn: u8,

    history: Vec<Move>,

    /// the kings' indices on the board
    kings: King,

    pub white_captures: Vec<PieceType>,
    pub black_captures: Vec<PieceType>,

    pub can_white_king_side_castle: bool,
    pub can_white_queen_side_castle: bool,

    pub can_black_king_side_castle: bool,
    pub can_black_queen_side_castle: bool,

    /// Record the number of each position.
    /// If any position happens 3 times, the game is declared draw
    unique_position_count: HashMap<String, u8>,

    half_moves: u8,
    full_moves: u8,

    lastest_en_passant_square: Option<u8>,

    last_turn: u8,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            board: [EMPTY; BOARD_SIZE as usize],
            turn: WHITE,
            history: vec![],
            kings: King { white: 1, black: 2 },
            white_captures: vec![],
            black_captures: vec![],
            can_white_king_side_castle: true,
            can_white_queen_side_castle: true,
            can_black_king_side_castle: true,
            can_black_queen_side_castle: true,
            unique_position_count: HashMap::new(),
            half_moves: 0,
            full_moves: 0,
            lastest_en_passant_square: None,
            last_turn: WHITE,
        }
    }

    pub fn move_piece(&mut self, from_idx: PieceIndex, to_idx: PieceIndex) {
        if self.is_on_board(to_idx) {
            let mut piece = self.get(from_idx);
            let to_piece = self.get(to_idx);
            let piece_without_color = self.remove_color(piece);

            let mut player_move = Move {
                from_idx,
                to_idx,
                from_piece: self.get(from_idx),
                to_piece,
                castle: false,
                captured: false,
                en_passant: false,
                en_passant_square: self.lastest_en_passant_square,
            };

            // preserve the en passant square if the turn hasn't changed when generating moves
            // TODO clear the square when we actually move a piece
            if self.turn != self.last_turn
                && piece_without_color != MOVED_PAWN
                && to_piece != EN_PASSANT_SQUARE
            {
                self.clear_latest_en_passant_square();
            }

            if piece_without_color == PAWN {
                // set the pawn to moved
                piece = piece | MOVED_MASK;
                // if it has moved 2 squares, update en passant pawns
                if to_idx.abs_diff(from_idx) == 32 {
                    // make the square behind the pawn an en passant square
                    let idx = match to_idx {
                        i if i > from_idx => from_idx + 16,
                        i if i < from_idx => from_idx - 16,
                        _ => panic!("could not calculate en passant squares"),
                    };

                    self.set(EN_PASSANT_SQUARE, idx);
                    self.lastest_en_passant_square = Some(idx);
                    // player_move.en_passant_square = self.lastest_en_passant_square;
                    // let left_idx = to_idx as i8 - 1;
                    // let right_idx = to_idx as i8 + 1;

                    // let indexes: [i8; 2] = [left_idx, right_idx];

                    // for index in indexes {
                    //     if self.is_on_board(index as PieceIndex) {
                    //         let piece = self.get(index as PieceIndex);
                    //         let is_enemy = !self.is_friendly(piece);
                    //         let piece_without_color = self.remove_color(piece);

                    //     }
                    // }
                }
            } else if piece_without_color == MOVED_PAWN {
                if to_piece == EN_PASSANT_SQUARE {
                    if to_idx > from_idx {
                        self.capture(self.get(to_idx - 16));
                        self.set(Piece::EMPTY, to_idx - 16);
                    } else {
                        self.capture(self.get(to_idx + 16));
                        self.set(Piece::EMPTY, to_idx + 16);
                    }

                    player_move.captured = true;
                    player_move.en_passant = true;
                }
            } else if piece_without_color == ROOK {
                piece = piece | MOVED_MASK;
            } else if piece_without_color == KING {
                self.update_kings_position(to_idx);
                piece = piece | MOVED_MASK;

                let mut can_king_side_castle = true;
                let mut can_queen_side_castle = true;

                if self.turn == WHITE {
                    can_king_side_castle = self.can_white_king_side_castle;
                    can_queen_side_castle = self.can_white_queen_side_castle;
                } else {
                    can_king_side_castle = self.can_black_king_side_castle;
                    can_queen_side_castle = self.can_black_queen_side_castle;
                }

                if can_king_side_castle || can_queen_side_castle {
                    if self.is_king_side_castling(from_idx, to_idx) {
                        self.set(Piece::ROOK | self.turn, to_idx - 1);
                        self.set(Piece::EMPTY, to_idx + 1);
                        player_move.castle = true;
                    } else if self.is_queen_side_castling(from_idx, to_idx) {
                        self.set(Piece::ROOK | self.turn, to_idx + 1);
                        self.set(Piece::EMPTY, to_idx - 2);
                        player_move.castle = true;
                    }
                }

                self.update_castling_rights(false, false);
            } else if piece_without_color == MOVED_KING {
                self.update_kings_position(to_idx);
                self.update_castling_rights(false, false);
            }

            // check if we are capturing a piece
            if to_piece != EMPTY && !self.is_friendly(to_piece) {
                self.capture(to_piece);
                player_move.captured = true;
            }

            self.set(piece, to_idx);
            self.set(Piece::EMPTY, from_idx);

            if self.turn == BLACK {
                self.full_moves += 1;
            }

            self.history.push(player_move);
            self.last_turn = self.turn
        } else {
            // panic!("illegal move!")
        }
    }

    /// Return the piece on the square
    fn get(&self, square_idx: PieceIndex) -> PieceIndex {
        if !self.is_on_board(square_idx) {
            panic!("square out of bound");
        }

        self.board[square_idx as usize]
    }

    /// Put a piece on a square
    pub fn set(&mut self, piece: PieceType, square_idx: PieceIndex) {
        if piece == KING {
            self.kings.white = square_idx;
        }

        if piece == BLACK_KING {
            self.kings.black = square_idx;
        }

        self.board[square_idx as usize] = piece;
    }

    pub fn moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let piece = self.get(square_idx);

        if !self.is_friendly(piece) {
            panic!("can't generate moves for enemy piece, set the turn correctly.");
        }

        let moves = match self.remove_color(piece) {
            PAWN => self.generate_pawn_moves(square_idx),
            MOVED_PAWN => self.generate_pawn_moves(square_idx),
            BISHOP => self.generate_sliding_moves(square_idx, BISHOP_DELTAS.to_vec()),
            ROOK => self.generate_sliding_moves(square_idx, ROOK_DELTAS.to_vec()),
            QUEEN => self.generate_sliding_moves(square_idx, QUEEN_DELTAS.to_vec()),
            KNIGHT => self.generate_knight_moves(square_idx),
            KING => self.generate_king_moves(square_idx, KING_DELTAS.to_vec()),
            MOVED_KING => self.generate_king_moves(square_idx, MOVED_KING_DELTAS.to_vec()),
            _ => vec![],
        };

        let mut legal_moves: Vec<u8> = vec![];

        for idx in moves {
            let destination_idx = idx;
            // play the move
            self.move_piece(square_idx, destination_idx as u8);

            if !self.in_check() {
                legal_moves.push(destination_idx);
            }

            // revert the move
            self.undo();
        }

        legal_moves
    }

    pub fn generate_pawn_moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let mut moves = vec![];

        let pawn = self.get(square_idx);

        let deltas = match pawn {
            PAWN => WHITE_PAWN_DELTAS,
            BLACK_PAWN => BLACK_PAWN_DELTAS,
            MOVED_PAWN => MOVED_WHITE_PAWN_DELTAS,
            MOVED_BLACK_PAWN => MOVED_BLACK_PAWN_DELTAS,
            _ => panic!("piece is not a pawn"),
        };

        let mut can_move_forward = true;

        for delta in deltas {
            if delta == 0 {
                continue;
            }

            let destination_idx = square_idx as i16 + delta as i16;
            let destination_idx = destination_idx as u8;

            if self.is_on_board(destination_idx) {
                let piece = self.get(destination_idx);

                // is it a diagonal move?
                if delta % 2 != 0 {
                    let is_enemy = !self.is_friendly(piece);
                    // if it is, is there an enemy piece it can capture?
                    if piece != EMPTY && is_enemy {
                        moves.push(destination_idx);
                    } else if piece == EN_PASSANT_SQUARE {
                        // en passant
                        moves.push(destination_idx);
                    }
                } else {
                    // pawn can only forward if it is not blocked by any piece
                    if piece != EMPTY {
                        can_move_forward = false;
                    }

                    if can_move_forward {
                        moves.push(destination_idx);
                    }
                }
            }
        }

        moves
    }

    pub fn generate_king_moves(&mut self, square_idx: PieceIndex, deltas: Vec<i8>) -> Vec<u8> {
        let mut moves: Vec<PieceIndex> = vec![];

        for delta in deltas {
            // convert to i16 to prevent overflow
            let destination_idx = (square_idx as i16 + delta as i16) as PieceIndex;

            if self.is_on_board(destination_idx) {
                let piece = self.get(destination_idx);
                let is_friendly = self.is_friendly(piece);

                if piece != EMPTY {
                    let is_castling_move = self.is_king_side_castling(square_idx, destination_idx)
                        || self.is_queen_side_castling(square_idx, destination_idx);

                    // we can only castle if the square is empty
                    if is_castling_move {
                        continue;
                    }

                    // if enemy piece, we can capture it
                    if !is_friendly {
                        moves.push(destination_idx);
                    }
                } else {
                    if self.is_castling(square_idx, destination_idx) {
                        let is_king_side_castling =
                            self.is_king_side_castling(square_idx, destination_idx);

                        let is_queen_side_castling =
                            self.is_queen_side_castling(square_idx, destination_idx);

                        if is_king_side_castling {
                            if self.turn == WHITE && !self.can_white_king_side_castle {
                                break;
                            }

                            if self.turn == BLACK && !self.can_black_king_side_castle {
                                break;
                            }
                        }

                        if is_queen_side_castling {
                            if self.turn == WHITE && !self.can_white_queen_side_castle {
                                break;
                            }
                            if self.turn == BLACK && !self.can_black_queen_side_castle {
                                break;
                            }
                        }

                        let rook_idx: PieceIndex = if is_king_side_castling {
                            destination_idx + 1
                        } else if is_queen_side_castling {
                            destination_idx - 2
                        } else {
                            panic!("failed to castle")
                        };

                        let piece = self.get(rook_idx);
                        let mut is_checked = false;
                        let mut idx = square_idx;

                        for _ in 0..2 {
                            if self.is_attacked(idx) {
                                is_checked = true;
                            }

                            if is_king_side_castling {
                                idx += 1;
                            }

                            if is_queen_side_castling {
                                idx -= 1;
                            }
                        }

                        if piece != EMPTY
                            && self.remove_color(piece) == ROOK
                            && self.is_friendly(piece)
                            && !is_checked
                        {
                            moves.push(destination_idx);
                        }

                        // if the piece is not an *unmoved* rook, then the king loses castling rights
                        if self.remove_color(piece) != ROOK && self.is_friendly(piece) {
                            if is_king_side_castling {
                                self.update_castling_rights(false, true);
                            }

                            if is_queen_side_castling {
                                self.update_castling_rights(true, false);
                            }
                        }
                    } else {
                        moves.push(destination_idx);
                    }
                }
            }
        }

        moves
    }

    pub fn generate_knight_moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let mut moves: Vec<PieceIndex> = vec![];

        for delta in KNIGHT_DELTAS {
            // convert to i16 to prevent overflow
            let destination_idx = square_idx as i16 + delta as i16;

            if self.is_on_board(destination_idx as u8) {
                let piece = self.get(destination_idx as PieceIndex);

                if piece != EMPTY && self.is_friendly(piece) {
                    continue;
                }

                moves.push(destination_idx as PieceIndex);
            }
        }

        moves
    }

    pub fn generate_sliding_moves(&mut self, square_idx: PieceIndex, deltas: Vec<i8>) -> Vec<u8> {
        let mut moves: Vec<PieceIndex> = vec![];

        for delta in deltas {
            // convert to i16 to prevent overflow
            let mut destination_idx = square_idx as i16 + delta as i16;

            while self.is_on_board(destination_idx as PieceIndex) {
                let piece = self.get(destination_idx as PieceIndex);

                if piece != EMPTY {
                    // if we encounter a friendly piece, we can't move there
                    if self.is_friendly(piece) {
                        break;
                    } else {
                        // if we encounter an enemy piece, we can capture it but cannot move further
                        moves.push(destination_idx as PieceIndex);
                        break;
                    }
                }

                moves.push(destination_idx as PieceIndex);

                // if the destination square is on the board, we keep searching in that direction until we go off the board
                destination_idx += delta as i16;
            }
        }

        moves
    }

    // TODO if this function causes any bugs down the road, just rewrite it by
    // saving a FEN string of the board everytime a move is made and use the previous FEN when undoing moves
    pub fn undo(&mut self) {
        if let Some(player_move) = self.history.pop() {
            if player_move.castle {
                let Move {
                    to_idx, from_idx, ..
                } = player_move;

                let king_side = self.is_king_side_castling(from_idx, to_idx);
                let queen_side = self.is_queen_side_castling(from_idx, to_idx);

                if king_side {
                    self.set(ROOK | self.turn, to_idx + 1);
                    self.set(EMPTY, from_idx + 1);

                    let queen_side_castling_rights = match self.turn {
                        WHITE => self.can_white_queen_side_castle,
                        BLACK => self.can_black_queen_side_castle,
                        _ => panic!("can't undo castling"),
                    };

                    self.update_castling_rights(true, queen_side_castling_rights);
                } else if queen_side {
                    self.set(ROOK | self.turn, to_idx - 2);
                    self.set(EMPTY, from_idx - 1);

                    let king_side_castling_rights = match self.turn {
                        WHITE => self.can_white_king_side_castle,
                        BLACK => self.can_black_king_side_castle,
                        _ => panic!("can't undo castling"),
                    };

                    self.update_castling_rights(king_side_castling_rights, true);
                }
            }

            let piece = self.remove_color(player_move.from_piece);
            if piece == KING || piece == MOVED_KING {
                let piece = self.remove_mask(piece, MOVED_MASK);

                if piece == KING {
                    self.kings.white = player_move.from_idx;
                }

                if piece == BLACK_KING {
                    self.kings.black = player_move.from_idx;
                }

                self.update_castling_rights(true, true)
            }

            if player_move.captured {
                if self.turn == WHITE {
                    self.white_captures.pop();
                } else {
                    self.black_captures.pop();
                }
            }

            // TODO test this
            if player_move.en_passant {
                let from_idx = player_move.from_idx;

                let idx = match player_move.to_idx {
                    i if i < from_idx => player_move.to_idx + 16,
                    i if i > from_idx => player_move.to_idx - 16,
                    _ => panic!("could not calculate en passant squares"),
                };

                self.set(EN_PASSANT_SQUARE, idx);

                let piece = self.remove_mask(player_move.from_piece, MOVED_MASK);

                if piece == MOVED_PAWN {
                    self.set(MOVED_BLACK_PAWN, player_move.to_idx + 16);
                } else if piece == MOVED_BLACK_PAWN {
                    self.set(MOVED_PAWN, player_move.to_idx - 16);
                }

                self.lastest_en_passant_square = player_move.en_passant_square;
            }

            if self.turn == BLACK {
                self.full_moves -= 1;
            }
            // move the pieces back to its original square
            self.set(player_move.from_piece, player_move.from_idx);
            self.set(player_move.to_piece, player_move.to_idx);
        }
    }

    /*
        black pieces = lowercase
        white = uppercase
        empty square = number
    */
    pub fn get_fen(&self) -> String {
        let mut fen = String::from("");

        let turn = self.turn();
        let mut castling_rights = String::new();
        let mut en_passant_square = match self.lastest_en_passant_square {
            Some(sq) => self.convert_index_algebraic_notation(sq),
            _ => String::from("-"),
        };
        let half_moves = self.half_moves;
        let full_moves = self.full_moves;

        if self.can_white_king_side_castle {
            castling_rights.push_str("K");
        }

        if self.can_white_queen_side_castle {
            castling_rights.push_str("Q");
        }

        if self.can_black_king_side_castle {
            castling_rights.push_str("k");
        }

        if self.can_black_queen_side_castle {
            castling_rights.push_str("q")
        }

        if castling_rights.is_empty() {
            castling_rights.push_str("-");
        }

        let mut empty_square: u8 = 0;
        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);

            if piece != EMPTY && piece != EN_PASSANT_SQUARE {
                if empty_square != 0 {
                    fen.push_str(empty_square.to_string().as_str());
                    empty_square = 0;
                }

                let color = self.get_color(piece);
                let original_piece = self.remove_mask(self.remove_color(piece), MOVED_MASK);

                if color == WHITE {
                    match original_piece {
                        PAWN => fen.push_str("P"),
                        ROOK => fen.push_str("R"),
                        KNIGHT => fen.push_str("N"),
                        BISHOP => fen.push_str("B"),
                        QUEEN => fen.push_str("Q"),
                        KING => fen.push_str("K"),
                        _ => panic!("error generating FEN"),
                    }
                } else {
                    match original_piece {
                        PAWN => fen.push_str("p"),
                        ROOK => fen.push_str("r"),
                        KNIGHT => fen.push_str("n"),
                        BISHOP => fen.push_str("b"),
                        QUEEN => fen.push_str("q"),
                        KING => fen.push_str("k"),
                        _ => panic!("error generating FEN"),
                    }
                }
            } else {
                empty_square += 1;
            }

            if (idx + 1) % 8 == 0 {
                if empty_square != 0 {
                    fen.push_str(empty_square.to_string().as_str());
                    empty_square = 0;
                }

                // if it is the last rank on board, no need to separate with /
                if idx != 119 {
                    fen.push('/');
                }
            }
        }

        vec![
            fen,
            turn.to_string(),
            castling_rights,
            en_passant_square,
            half_moves.to_string(),
            full_moves.to_string(),
        ]
        .join(" ")
    }

    pub fn load_fen(&mut self, fen: String) {
        let fen_parts: Vec<&str> = fen.split(' ').collect();

        let ranks: Vec<&str> = fen_parts[0].split("/").collect();

        let mut idx: usize = 0;
        for rank in ranks {
            for piece in rank.chars() {
                match piece {
                    'p' => self.set(BLACK_PAWN, ACTUAL_BOARD[idx]),
                    'r' => self.set(BLACK_ROOK, ACTUAL_BOARD[idx]),
                    'n' => self.set(BLACK_KNIGHT, ACTUAL_BOARD[idx]),
                    'b' => self.set(BLACK_BISHOP, ACTUAL_BOARD[idx]),
                    'q' => self.set(BLACK_QUEEN, ACTUAL_BOARD[idx]),
                    'k' => self.set(BLACK_KING, ACTUAL_BOARD[idx]),
                    'P' => self.set(PAWN, ACTUAL_BOARD[idx]),
                    'R' => self.set(ROOK, ACTUAL_BOARD[idx]),
                    'N' => self.set(KNIGHT, ACTUAL_BOARD[idx]),
                    'B' => self.set(BISHOP, ACTUAL_BOARD[idx]),
                    'Q' => self.set(QUEEN, ACTUAL_BOARD[idx]),
                    'K' => self.set(KING, ACTUAL_BOARD[idx]),
                    '1'..='8' => idx += piece.to_digit(10).unwrap() as usize - 1,
                    _ => panic!("can't load fen pieces"),
                }

                idx += 1;
            }
        }

        // set turn
        match fen_parts[1] {
            "w" => self.set_turn(WHITE),
            "b" => self.set_turn(WHITE),
            _ => panic!("can't load fen turn"),
        }

        // castling rights
        for castling_right in fen_parts[2].chars() {
            match castling_right {
                'K' => {
                    self.can_white_king_side_castle = true;
                }
                'Q' => {
                    self.can_white_queen_side_castle = true;
                }
                'k' => {
                    self.can_black_king_side_castle = true;
                }
                'q' => {
                    self.can_black_queen_side_castle = true;
                }

                '-' => {
                    self.can_white_king_side_castle = false;
                    self.can_white_queen_side_castle = false;
                    self.can_black_king_side_castle = false;
                    self.can_black_queen_side_castle = false;
                }
                _ => panic!("cant load fen castling rights"),
            }
        }

        // en passant square
        let square = fen_parts[3];

        match square {
            // no en passant square
            "-" => {}
            _ => {
                let idx = self.convert_algebraic_notation_to_index(square) as usize;

                self.set(EN_PASSANT_SQUARE, ACTUAL_BOARD[idx]);
            }
        }

        self.half_moves = fen_parts[4].parse().unwrap();
        self.full_moves = fen_parts[5].parse().unwrap();
    }

    /// Return true or false if the color to move is in check
    pub fn in_check(&self) -> bool {
        let king_idx = match self.turn {
            WHITE => self.kings.white,
            BLACK => self.kings.black,
            _ => panic!("Turn cannot be determined when checking if king is attacked"),
        };

        return self.is_attacked(king_idx);
    }

    pub fn is_checkmate(&mut self) -> bool {
        let mut no_legal_moves = true;

        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);

            if self.is_friendly(piece) {
                let moves = self.moves(idx);

                if moves.len() > 0 {
                    no_legal_moves = false;
                }
            }
        }

        self.in_check() && no_legal_moves
    }

    pub fn is_draw(&mut self) -> bool {
        self.is_stalemate() || self.is_threefold_repetition() || self.is_50_moves_rule()
    }

    /// stalemate happens when a player has no legal moves and is not in check
    pub fn is_stalemate(&mut self) -> bool {
        let mut no_legal_moves = true;

        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);

            if (piece != EMPTY || piece != EN_PASSANT_SQUARE) && self.is_friendly(piece) {
                let moves = self.moves(idx);

                if moves.len() > 0 {
                    no_legal_moves = false;
                }
            }
        }

        !self.in_check() && no_legal_moves
    }

    // TODO fix this lol
    // convert every move to FEN, store it as a unique key in a hashmap
    // if the value >= 3, then it is threefold repetition
    pub fn is_threefold_repetition(&mut self) -> bool {
        true
    }

    pub fn is_50_moves_rule(&mut self) -> bool {
        true
    }

    /*
        If both sides have any one of the following, and there are no pawns or other pieces on the board:

        A lone king
        a king and bishop
        a king and knight

        a king and two knights vs a lone king = draw

        accoring to https://support.chess.com/article/128-what-does-insufficient-mating-material-mean
    */
    pub fn is_insufficient_materials(&mut self) -> bool {
        let mut friendly_knights = 0;
        let mut friendly_bishops = 0;
        let mut enemy_knights = 0;
        let mut enemy_bishops = 0;

        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);

            if piece != EMPTY {
                let piece_without_color = self.remove_color(piece);

                if piece_without_color == PAWN
                    || piece_without_color == ROOK
                    || piece_without_color == QUEEN
                {
                    return false;
                }

                if self.is_friendly(piece) {
                    if piece_without_color == KNIGHT {
                        friendly_knights += 1;
                    }

                    if piece_without_color == BISHOP {
                        friendly_bishops += 1;
                    }
                } else {
                    if piece_without_color == KNIGHT {
                        enemy_knights += 1;
                    }

                    if piece_without_color == BISHOP {
                        enemy_bishops += 1;
                    }
                }
            }
        }

        // king vs king
        if friendly_knights == 0
            && friendly_bishops == 0
            && enemy_knights == 0
            && enemy_bishops == 0
        {
            return true;
        }

        if friendly_knights == 0 && enemy_knights == 0 {
            if friendly_bishops == 2 || enemy_bishops == 2 {
                return false;
            }

            return true;
        }

        if friendly_bishops == 0 && enemy_bishops == 0 {
            if friendly_knights == 2 || enemy_knights == 2 {
                return false;
            }

            return true;
        }

        return false;
    }

    pub fn is_attacked(&self, square_idx: PieceIndex) -> bool {
        let mut is_attacked = false;
        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);
            let attacker_idx = idx;
            let defender_idx = square_idx;
            let defender_piece = self.get(defender_idx);

            if self.is_friendly(piece) || piece == EMPTY {
                continue;
            }

            let diff = (defender_idx as i16 - attacker_idx as i16) + 119;

            let attack_bits_mask = ATTACKS[diff as usize];

            if attack_bits_mask != 0 {
                // although the king can be attacked from a particular square, we also
                // have to take into account if that piece can attack from there
                let piece = self.get(attacker_idx);

                // remove the color mask
                let original_piece = self.remove_mask(self.remove_color(piece), MOVED_MASK);

                // check if that piece can attack from that particular square
                if (original_piece & attack_bits_mask) == original_piece {
                    if original_piece == KNIGHT {
                        return true;
                    } else if original_piece == PAWN {
                        let piece = self.remove_mask(piece, MOVED_MASK);
                        is_attacked = piece & attack_bits_mask == piece;
                    } else {
                        // check if there is a piece standing in the way of the attack
                        let delta = DELTAS[diff as usize];

                        let mut destination_idx = attacker_idx as i16 + delta as i16;

                        while self.is_on_board(destination_idx as PieceIndex) {
                            let piece = self.get(destination_idx as PieceIndex);

                            if piece == defender_piece
                                && destination_idx as PieceIndex == defender_idx
                            {
                                is_attacked = true;
                            } else {
                                // check if there is a piece blocking the attack
                                if piece != EMPTY {
                                    break;
                                }
                            }

                            destination_idx += delta as i16;
                        }
                    }
                }
            }
        }
        return is_attacked;
    }

    pub fn turn(&self) -> char {
        match self.turn {
            WHITE => 'w',
            BLACK => 'b',
            _ => panic!("Unknown turn"),
        }
    }

    /// 0 = white, 128 = black
    pub fn set_turn(&mut self, turn: u8) {
        if turn == WHITE {
            self.turn = WHITE;
        } else {
            self.turn = BLACK;
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn convert_algebraic_notation_to_index(&self, notation: &str) -> u8 {
        let mut parts = notation.chars();

        let file = parts.next().unwrap();
        let rank = (parts.next().unwrap().to_digit(10).unwrap() - 1) as u8;

        let file = FILES.iter().position(|f| f.eq(&file)).unwrap() as u8;

        // we minus rank from 7 because the board is reversed (upside down)
        // so for example, for "e7", the rank 7 is rank 1 on our board
        (8 * (7 - rank) + file) as u8
    }

    pub fn convert_index_algebraic_notation(&self, index: u8) -> String {
        let file = index & 7;
        let rank = 8 - ((index >> 4) + 1) + 1;

        let file_letter = FILES[file as usize];

        let mut notation = String::new();

        notation.push(file_letter);
        notation.push_str(rank.to_string().as_str());

        notation
    }

    fn change_turn(&mut self) {
        if self.turn == WHITE {
            self.set_turn(BLACK);
        } else {
            self.set_turn(WHITE);
        }
    }

    fn clear_latest_en_passant_square(&mut self) {
        if let Some(idx) = self.lastest_en_passant_square {
            let piece = self.get(idx);

            if piece == EN_PASSANT_SQUARE {
                self.set(EMPTY, idx);
                self.lastest_en_passant_square = None;
            }
        }
    }

    fn update_castling_rights(&mut self, k: bool, q: bool) {
        if self.turn == WHITE {
            self.can_white_king_side_castle = k;
            self.can_white_queen_side_castle = q;
        } else {
            self.can_black_king_side_castle = k;
            self.can_black_queen_side_castle = q;
        }
    }

    fn get_color(&self, piece: PieceType) -> PieceType {
        piece & COLOR_MASK
    }

    fn remove_color(&self, piece: PieceType) -> PieceType {
        self.remove_mask(piece, COLOR_MASK)
    }

    fn is_castling(&self, from: PieceIndex, to: PieceIndex) -> bool {
        let can_castle = match self.turn {
            WHITE => self.can_white_queen_side_castle || self.can_white_king_side_castle,
            BLACK => self.can_black_queen_side_castle || self.can_black_king_side_castle,
            _ => panic!("can't check castling rights"),
        };

        (self.is_queen_side_castling(from, to) || self.is_king_side_castling(from, to))
            && can_castle
    }

    fn is_queen_side_castling(&self, from: PieceIndex, to: PieceIndex) -> bool {
        to as i8 - from as i8 == -2
    }

    fn is_king_side_castling(&self, from: PieceIndex, to: PieceIndex) -> bool {
        to as i8 - from as i8 == 2
    }

    fn update_kings_position(&mut self, new_idx: PieceIndex) {
        if self.turn == WHITE {
            self.kings.white = new_idx;
        } else {
            self.kings.black = new_idx;
        }
    }

    fn capture(&mut self, piece: PieceType) {
        if self.turn == WHITE {
            self.white_captures.push(piece);
        } else {
            self.black_captures.push(piece);
        }
    }

    fn remove_mask(&self, piece: PieceType, mask: u8) -> u8 {
        (piece | mask) ^ mask
    }
    /// a piece is friendly if its color matches the current player's turn color
    fn is_friendly(&self, piece: PieceType) -> bool {
        piece & COLOR_MASK == self.turn
    }

    fn is_on_board(&self, square_idx: PieceIndex) -> bool {
        (square_idx) & 0x88 == 0
    }
}

// TODO: sort the moves array in tests to ensure they match

#[cfg(test)]
mod bishop {
    use super::*;

    #[test]
    fn bishop_can_move_freely_if_king_is_not_checked() {
        let mut chess = Chess::new();

        chess.set(BISHOP, 52);
        chess.set(KING, 55);
        chess.set(BISHOP | BLACK, 86);

        let moves = chess.moves(52);
        let correct_moves = [69, 86, 67, 82, 97, 112, 35, 18, 1, 37, 22, 7];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 0);
        chess.set(KING, 1);
        chess.set(BISHOP | BLACK, 7);

        let moves = chess.moves(0);
        let correct_moves = [17, 34, 51, 68, 85, 102, 119];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 103);
        chess.set(KING | BLACK, 81);
        chess.set(BISHOP | BLACK, 36);

        let moves = chess.moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 103);
        chess.set(KING | BLACK, 81);
        chess.set(BISHOP | BLACK, 36);

        let moves = chess.moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 0);
        chess.set(KING | BLACK, 7);
        chess.set(BISHOP | BLACK, 112);

        let moves = chess.moves(112);
        let correct_moves = [97, 82, 67, 52, 37, 22];

        assert!(moves.iter().eq(correct_moves.iter()));
    }

    #[test]
    fn bishop_can_not_move_because_king_is_checked() {
        let mut chess = Chess::new();

        chess.set(BISHOP, 117);
        chess.set(KING, 112);
        chess.set(BLACK_BISHOP, 97);

        let moves = chess.moves(117);

        assert!(moves.len() == 0);

        chess.clear();

        chess.set(BISHOP, 119);
        chess.set(KING, 7);
        chess.set(BISHOP | BLACK, 22);

        let moves = chess.moves(119 as u8);
        assert!(moves.len() == 0);

        chess.clear();

        chess.set(BISHOP, 67);
        chess.set(KING, 0);
        chess.set(BISHOP | BLACK, 17);

        let moves = chess.moves(67 as u8);
        assert!(moves.len() == 0);

        chess.clear();

        //======= BLACK =======
        chess.set_turn(BLACK);

        chess.set(BISHOP, 0);
        chess.set(KING | BLACK, 17);
        chess.set(BISHOP | BLACK, 51);

        let moves = chess.moves(51);
        assert!(moves.len() == 0);

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 67);
        chess.set(KING | BLACK, 112);
        chess.set(BISHOP | BLACK, 100);

        let moves = chess.moves(100);
        assert!(moves.len() == 0);

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 118);
        chess.set(KING | BLACK, 16);
        chess.set(BISHOP | BLACK, 21);

        let moves = chess.moves(21);
        assert!(moves.len() == 0);
    }

    #[test]
    fn bishop_can_take_enemy_piece_to_stop_check() {
        let mut chess = Chess::new();

        chess.set(BISHOP, 2);
        chess.set(KING, 0);
        chess.set(BISHOP | BLACK, 17);

        let moves = chess.moves(2 as u8);
        let correct_moves = [17];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 99);
        chess.set(KING, 37);
        chess.set(BISHOP | BLACK, 54);

        let moves = chess.moves(99 as u8);
        let correct_moves = [54];

        assert!(moves.iter().eq(correct_moves.iter()));

        // ====== BLACK =====

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 32);
        chess.set(KING | BLACK, 66);
        chess.set(BISHOP | BLACK, 17);

        let moves = chess.moves(17);
        let correct_moves = [32];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 67);
        chess.set(KING | BLACK, 7);
        chess.set(BISHOP | BLACK, 118);

        let moves = chess.moves(118);
        let correct_moves = [67];

        assert!(moves.iter().eq(correct_moves.iter()));
    }

    #[test]
    fn bishop_can_move_freely_if_king_is_shielded_from_check() {
        let mut chess = Chess::new();
        chess.set(BISHOP, 51);
        chess.set(KING, 7);
        chess.set(PAWN, 37);
        chess.set(BISHOP | BLACK, 67);

        let moves = chess.moves(51);
        let correct_moves = [68, 85, 102, 119, 66, 81, 96, 34, 17, 0, 36, 21, 6];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 118);
        chess.set(KING, 119);
        chess.set(PAWN, 102);
        chess.set(BISHOP | BLACK, 85);

        let moves = chess.moves(118);
        let correct_moves = [101, 84, 67, 50, 33, 16, 103];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 66);
        chess.set(KING, 17);
        chess.set(PAWN, 34);
        chess.set(BISHOP | BLACK, 51);

        let moves = chess.moves(66);
        let correct_moves = [83, 100, 117, 81, 96, 49, 32, 51];

        assert!(moves.iter().eq(correct_moves.iter()));

        // ==== BLACK ====
        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 64);
        chess.set(KING | BLACK, 4);
        chess.set(PAWN | BLACK, 19);
        chess.set(BISHOP | BLACK, 84);

        let moves = chess.moves(84);
        let correct_moves = [101, 118, 99, 114, 67, 50, 33, 16, 69, 54, 39];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 53);
        chess.set(KING | BLACK, 87);
        chess.set(PAWN | BLACK, 70);
        chess.set(BISHOP | BLACK, 98);

        let moves = chess.moves(98);
        let correct_moves = [115, 113, 81, 64, 83, 68, 53];

        assert!(moves.iter().eq(correct_moves.iter()));
    }
}

#[cfg(test)]
mod pawn {
    use super::*;

    #[test]
    fn pawn_valid_moves() {
        let mut chess = Chess::new();

        chess.set_turn(BLACK);
        chess.set(Piece::BLACK_PAWN, 98);
        let moves = chess.moves(98);
        let correct_moves = [114];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set_turn(BLACK);
        chess.set(Piece::BLACK_PAWN, 2);
        let moves = chess.moves(2);
        let correct_moves = [18, 34];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::PAWN, 34);
        let moves = chess.moves(34);
        let correct_moves = [18, 2];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::PAWN, 23);
        let moves = chess.moves(23);
        let correct_moves = [7];
        assert!(moves.iter().eq(correct_moves.iter()));
    }
}
