use crate::chess::Piece::*;
use crate::errors::*;
use regex::Regex;
use std::collections::HashMap;
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

const BOARD_MAP: [u8; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23, 32, 33, 34, 35, 36, 37, 38, 39, 48, 49,
    50, 51, 52, 53, 54, 55, 64, 65, 66, 67, 68, 69, 70, 71, 80, 81, 82, 83, 84, 85, 86, 87, 96, 97,
    98, 99, 100, 101, 102, 103, 112, 113, 114, 115, 116, 117, 118, 119,
];

const FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

fn is_number(arg: &str) -> bool {
    arg.chars()
        .collect::<Vec<char>>()
        .iter()
        .all(|&c| c.is_digit(10))
}

#[derive(Debug)]
struct HistoryEntry {
    from_idx: PieceIndex,
    to_idx: PieceIndex,
    from_piece: PieceType,
    to_piece: PieceType,
    castle: bool,
    capture: bool,
    en_passant_capture: bool,
    en_passant_move: bool,
    promotion: bool,
    half_moves: u8,
    full_moves: u8,
    en_passant_square: Option<String>,
    can_white_king_side_castle: bool,
    can_white_queen_side_castle: bool,
    can_black_king_side_castle: bool,
    can_black_queen_side_castle: bool,
}

#[derive(Debug)]
struct Move {
    from: PieceIndex,
    to: PieceIndex,
    // in_check: bool,
    // opponent_in_check: bool,
}

// impl Vec<Move> {}

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

    history: Vec<HistoryEntry>,

    /// the kings' indices on the board
    kings: King,

    pub white_captures: Vec<PieceType>,
    pub black_captures: Vec<PieceType>,

    // TODO use bits for these so we only have to keep track of two fields
    pub can_white_king_side_castle: bool,
    pub can_white_queen_side_castle: bool,

    pub can_black_king_side_castle: bool,
    pub can_black_queen_side_castle: bool,

    /// Record the number of each position.
    /// If any position happens 3 times, the game is declared draw
    unique_positions: HashMap<String, u8>,

    half_moves: u8,
    full_moves: u8,

    lastest_en_passant_square: Option<String>,

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
            unique_positions: HashMap::new(),
            half_moves: 0,
            full_moves: 0,
            lastest_en_passant_square: None,
            last_turn: WHITE,
        }
    }

    pub fn move_piece(&mut self, move_notation: &str) -> Result<String, MoveError> {
        let parts: Vec<char> = move_notation.chars().collect();
        let move_regex =
            Regex::new(r"^([KQRBN])?([a-h]|[1-8])?(x)?([a-h])([1-8])(=)?([KQRBN])?([+#])?$")
                .unwrap();

        // TODO is there anyway we can reduce the noise?

        // king-side castle
        if move_notation == "O-O" {
            if self.turn == WHITE {
                if !self.can_white_king_side_castle {
                    return Err(MoveError::IllegalKingSideCastle);
                }

                let legal_moves = self.inner_moves(self.kings.white);

                // let m = Move {
                //     from: self.kings.white,
                //     to: 118,
                //     opponent_in_check: false,
                // };

                if legal_moves.contains(&118) {
                    self.inner_move_piece(self.kings.white, 118);
                } else {
                    return Err(MoveError::IllegalKingSideCastle);
                }
            } else {
                if !self.can_black_king_side_castle {
                    return Err(MoveError::IllegalKingSideCastle);
                }

                let legal_moves = self.inner_moves(self.kings.black);

                if legal_moves.contains(&6) {
                    self.inner_move_piece(self.kings.black, 6);
                } else {
                    return Err(MoveError::IllegalKingSideCastle);
                }
            }
        // queen side castle
        } else if move_notation == "O-O-O" {
            if self.turn == WHITE {
                if !self.can_white_queen_side_castle {
                    return Err(MoveError::IllegalQueenSideCastle);
                }

                let legal_moves = self.inner_moves(self.kings.white);

                if legal_moves.contains(&114) {
                    self.inner_move_piece(self.kings.white, 114);
                } else {
                    return Err(MoveError::IllegalQueenSideCastle);
                }
            } else {
                if !self.can_black_queen_side_castle {
                    return Err(MoveError::IllegalQueenSideCastle);
                }

                let legal_moves = self.inner_moves(self.kings.black);

                if legal_moves.contains(&2) {
                    self.inner_move_piece(self.kings.black, 2);
                } else {
                    return Err(MoveError::IllegalQueenSideCastle);
                }
            }
        } else {
            // TODO error for this
            let captures = move_regex
                .captures(move_notation)
                .expect("Invalid move notation");

            let piece_identifer = captures.get(2).map_or("", |m| m.as_str());
            let is_capture = captures.get(3).map_or(false, |_| true);
            let to_file = captures.get(4).map_or("", |m| m.as_str());
            let to_rank = captures.get(5).map_or("", |m| m.as_str());
            let is_promotion = captures.get(6).map_or(false, |_| true);
            let promotion_piece = captures.get(7).map_or("", |m| m.as_str());

            let mut from_idx: Option<PieceIndex> = None;
            let idx = self
                .convert_algebraic_notation_to_index(format!("{}{}", to_file, to_rank).as_str());

            let to_idx = BOARD_MAP[idx as usize];
            let target_piece = self.get(to_idx);

            // en passant only if piece is pawn
            if is_capture
                && ((self.is_friendly(target_piece) && target_piece != EN_PASSANT_SQUARE)
                    || target_piece == EMPTY)
            {
                return Err(MoveError::IllegalCapture);
            }

            let mut target_file = None;
            let mut target_rank = None;

            if !piece_identifer.is_empty() {
                if is_number(piece_identifer) {
                    target_rank = Some(piece_identifer.parse::<u8>().unwrap());
                } else {
                    target_file =
                        Some(FILES.iter().position(|f| f.eq(&piece_identifer)).unwrap() as u8);
                }
            }

            if parts[0].is_uppercase() {
                // [a-h][file-rank-identifier][a-h][file]
                // [a-h]x[a-h][file]
                // [a-h][file-rank-identifier]x[a-h][file]

                let mut count = 0;
                let target_type = match parts[0] {
                    'K' => KING,
                    'Q' => QUEEN,
                    'R' => ROOK,
                    'B' => BISHOP,
                    'N' => KNIGHT,
                    _ => {
                        return Err(MoveError::UnknownPiece);
                    }
                };

                for idx in 0..BOARD_SIZE {
                    if !self.is_on_board(idx) {
                        continue;
                    }

                    let piece = self.get(idx);
                    let piece_type = self.get_type(piece);

                    if piece_type == target_type && self.is_friendly(piece) {
                        count += 1;

                        // TODO use .moves instead of .inner_moves so we don't have to convert the index
                        let _moves = self.inner_moves(idx);
                        let file = idx & 7;
                        let rank = 8 - ((idx >> 4) + 1) + 1;

                        if !_moves.contains(&to_idx) {
                            continue;
                        }

                        if let Some(target_file) = target_file {
                            if file == target_file {
                                if count >= 2 && _moves.contains(&to_idx) {
                                    return Err(MoveError::AmbiguousMoveNotation);
                                }
                                from_idx = Some(idx);
                            }
                        } else if let Some(target_rank) = target_rank {
                            if rank == target_rank {
                                if count >= 2 && _moves.contains(&to_idx) {
                                    return Err(MoveError::AmbiguousMoveNotation);
                                }
                                from_idx = Some(idx);
                            }
                        } else {
                            if count >= 2 && _moves.contains(&to_idx) && from_idx != None {
                                // if the second piece can also move to that location, then we panic
                                // because we don't know which piece to move
                                return Err(MoveError::AmbiguousMoveNotation);
                            }

                            from_idx = Some(idx);
                        }
                    }
                }

                if let Some(from_idx) = from_idx {
                    self.inner_move_piece(from_idx, to_idx)
                } else {
                    return Err(MoveError::InvalidPieceToMove);
                }
            } else {
                let rank = 8 - ((to_idx >> 4) + 1) + 1;

                if is_promotion && (rank > 1 && rank < 8) {
                    return Err(MoveError::InvalidPromotion);
                }

                if rank == 0 || rank == 8 {
                    if !is_promotion {
                        return Err(MoveError::InvalidPromotion);
                    }

                    if is_promotion {
                        if promotion_piece.is_empty() {
                            return Err(MoveError::InvalidPromotion);
                        }
                    }
                }

                // pawn move
                for idx in 0..BOARD_SIZE {
                    if !self.is_on_board(idx) {
                        continue;
                    }

                    let piece = self.get(idx);
                    let piece_type = self.get_type(piece);

                    if piece_type == PAWN && self.is_friendly(piece) {
                        let _moves = self.inner_moves(idx);

                        if !_moves.contains(&to_idx) {
                            // println!("{} {:?}", idx, _moves);
                            continue;
                        }
                        from_idx = Some(idx);
                    }
                }

                if let Some(from_idx) = from_idx {
                    self.inner_move_piece(from_idx, to_idx);

                    if is_promotion {
                        let piece = match promotion_piece {
                            "K" => KING,
                            "Q" => QUEEN,
                            "R" => ROOK,
                            "B" => BISHOP,
                            "N" => KNIGHT,
                            _ => return Err(MoveError::InvalidPromotion),
                        };

                        self.set(piece, to_idx);
                    }
                } else {
                    return Err(MoveError::InvalidPieceToMove);
                }
            }
        }

        if self.turn == BLACK {
            self.full_moves += 1;
        }

        let fen = self.get_fen();
        let position = fen.split(" ").collect::<Vec<&str>>()[0];

        *self
            .unique_positions
            .entry(position.to_string())
            .or_insert(0) += 1;
        self.change_turn();
        self.update_castling_rights();

        if self.is_draw() {
            println!("game drawn");
        }

        let mut new_notation = move_notation.to_string();

        if self.is_checkmate() {
            println!("game over by checkmate");
            new_notation.push('#');
        } else if self.in_check() {
            new_notation.push('+');
        }

        Ok(new_notation)
    }

    pub fn inner_move_piece(&mut self, from_idx: PieceIndex, to_idx: PieceIndex) {
        if self.is_on_board(to_idx) {
            let mut piece = self.get(from_idx);

            if piece == EMPTY || piece == EN_PASSANT_SQUARE {
                panic!("can't move an empty square");
            }

            let to_piece = self.get(to_idx);
            let piece_type = self.remove_color(piece);

            let mut history_entry = HistoryEntry {
                from_idx,
                to_idx,
                from_piece: piece,
                to_piece,
                castle: false,
                capture: false,
                en_passant_capture: false,
                en_passant_move: false,
                promotion: false,
                half_moves: self.half_moves,
                full_moves: self.full_moves,
                en_passant_square: self.lastest_en_passant_square.clone(),
                can_white_king_side_castle: self.can_white_king_side_castle,
                can_white_queen_side_castle: self.can_white_queen_side_castle,
                can_black_king_side_castle: self.can_black_king_side_castle,
                can_black_queen_side_castle: self.can_black_queen_side_castle,
            };

            self.clear_latest_en_passant_square();

            if piece_type == PAWN {
                // set the pawn to moved
                piece = piece | MOVED_MASK;
                // if it has moved 2 squares, update en passant square
                if to_idx.abs_diff(from_idx) == 32 {
                    // make the square behind the pawn an en passant square
                    let idx = match to_idx {
                        i if i > from_idx => from_idx + 16,
                        i if i < from_idx => from_idx - 16,
                        _ => panic!("could not calculate en passant squares"),
                    };

                    self.set(EN_PASSANT_SQUARE, idx);
                    self.lastest_en_passant_square =
                        Some(self.convert_index_algebraic_notation(idx));
                    history_entry.en_passant_move = true;
                }

                self.reset_half_moves();
            } else if piece_type == MOVED_PAWN {
                if to_piece == EN_PASSANT_SQUARE {
                    if self.turn == BLACK {
                        self.capture(self.get(to_idx - 16));
                        self.set(Piece::EMPTY, to_idx - 16);
                    } else {
                        self.capture(self.get(to_idx + 16));
                        self.set(Piece::EMPTY, to_idx + 16);
                    }

                    history_entry.capture = true;
                    history_entry.en_passant_capture = true;
                    self.reset_half_moves();
                }

                let rank = 8 - ((to_idx >> 4) + 1) + 1;

                if rank == 1 || rank == 8 {
                    history_entry.promotion = true;
                }
                self.reset_half_moves();
            } else if piece_type == KING {
                self.update_kings_position(to_idx);

                let (can_king_side_castle, can_queen_side_castle) = self.get_castling_rights();

                if can_king_side_castle || can_queen_side_castle {
                    if self.is_king_side_castling(from_idx, to_idx) {
                        self.set(Piece::ROOK | self.turn, to_idx - 1);
                        self.set(Piece::EMPTY, to_idx + 1);
                        history_entry.castle = true;
                    } else if self.is_queen_side_castling(from_idx, to_idx) {
                        self.set(Piece::ROOK | self.turn, to_idx + 1);
                        self.set(Piece::EMPTY, to_idx - 2);
                        history_entry.castle = true;
                    }
                }
            } else if piece_type == MOVED_KING {
                self.update_kings_position(to_idx);
            } else {
                self.half_moves += 1;
            }

            // check if we are capturing a piece
            if to_piece != EMPTY && to_piece != EN_PASSANT_SQUARE && !self.is_friendly(to_piece) {
                self.capture(to_piece);
                history_entry.capture = true;
                self.reset_half_moves();
            }

            self.set(piece, to_idx);
            self.set(Piece::EMPTY, from_idx);

            self.history.push(history_entry);
        } else {
            // panic!("illegal move!")
        }
    }

    /// Return the piece on the square
    fn get(&self, square_idx: PieceIndex) -> PieceIndex {
        if !self.is_on_board(square_idx) || square_idx > 150 {
            panic!("square out of bound");
        }

        self.board[square_idx as usize]
    }

    /// Put a piece on a square
    pub fn set(&mut self, piece: PieceType, square_idx: PieceIndex) {
        if !self.is_on_board(square_idx) || square_idx > 150 {
            panic!("square out of bound");
        }

        if piece == KING {
            self.kings.white = square_idx;
        }

        if piece == BLACK_KING {
            self.kings.black = square_idx;
        }

        self.board[square_idx as usize] = piece;

        // self.update_castling_rights();
    }

    pub fn moves(&mut self, square: &str) -> Vec<String> {
        let square_idx = BOARD_MAP[self.convert_algebraic_notation_to_index(square) as usize];

        if !self.is_on_board(square_idx) {
            panic!("invalid square");
        }

        let moves = self.inner_moves(square_idx);

        let piece = self.get(square_idx);
        let piece_type = self.get_type(piece);

        let prefix = match piece_type {
            KING => "K",
            QUEEN => "Q",
            ROOK => "R",
            BISHOP => "B",
            KNIGHT => "N",
            PAWN => "",
            _ => {
                panic!("invalid piece")
            }
        };

        moves
            .iter()
            .map(|m| {
                // TODO: use a Move struct to make our lives easier

                if *m == 118 && piece_type == KING {
                    return String::from("O-O");
                } else if *m == 114 && piece_type == KING {
                    return String::from("O-O-O");
                } else {
                    let to = self.get(*m);

                    if to == EN_PASSANT_SQUARE {
                        let file = square_idx & 7;

                        return format!(
                            "{}x{}",
                            FILES[file as usize],
                            self.convert_index_algebraic_notation(*m)
                        );
                    }

                    if to != EMPTY && !self.is_friendly(to) {
                        return format!("{}x{}", prefix, self.convert_index_algebraic_notation(*m));
                    }

                    return format!("{}{}", prefix, self.convert_index_algebraic_notation(*m));
                }
            })
            .collect()
    }

    pub fn inner_moves(&mut self, square_idx: PieceIndex) -> Vec<PieceIndex> {
        let piece = self.get(square_idx);

        if !self.is_friendly(piece) {
            panic!("can't generate inner_moves for enemy piece, set the turn correctly.");
        }

        let inner_moves = match self.remove_color(piece) {
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

        let mut legal_moves: Vec<PieceIndex> = vec![];

        for idx in inner_moves {
            let to_idx = idx;
            // play the move
            self.inner_move_piece(square_idx, to_idx as u8);

            if !self.in_check() {
                // println!("{:?}", self.board);
                legal_moves.push(to_idx);
                // legal_moves.push(Move {
                //     // Optimize this later?
                //     // opponent_in_check: self.opponent_in_check(),
                //     from: square_idx,
                //     to: to_idx,
                // });
            }

            // revert the move
            self.undo();
        }

        legal_moves
    }

    pub fn generate_pawn_moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let mut inner_moves = vec![];

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
                        inner_moves.push(destination_idx);
                    } else if piece == EN_PASSANT_SQUARE {
                        if self.turn == WHITE {
                            let below = self.get(destination_idx + 16);
                            if !self.is_friendly(below) && self.get_type(below) == PAWN {
                                inner_moves.push(destination_idx);
                            }
                        } else {
                            let above = self.get(destination_idx - 16);
                            if !self.is_friendly(above) && self.get_type(above) == PAWN {
                                inner_moves.push(destination_idx);
                            }
                        }
                    }
                } else {
                    // pawn can only forward if it is not blocked by any piece
                    if piece != EMPTY {
                        can_move_forward = false;
                    }

                    if can_move_forward {
                        inner_moves.push(destination_idx);
                    }
                }
            }
        }

        inner_moves
    }

    pub fn generate_king_moves(&mut self, square_idx: PieceIndex, deltas: Vec<i8>) -> Vec<u8> {
        let mut inner_moves: Vec<PieceIndex> = vec![];

        for delta in deltas {
            // convert to i16 to prevent overflow
            let destination_idx = (square_idx as i16 + delta as i16) as PieceIndex;

            if self.is_on_board(destination_idx) {
                let piece = self.get(destination_idx);
                let is_friendly = self.is_friendly(piece);

                if piece != EMPTY && piece != EN_PASSANT_SQUARE {
                    let is_castling_move = self.is_king_side_castling(square_idx, destination_idx)
                        || self.is_queen_side_castling(square_idx, destination_idx);

                    // we can only castle if the square is empty
                    if is_castling_move {
                        continue;
                    }

                    // if enemy piece, we can capture it
                    if !is_friendly {
                        inner_moves.push(destination_idx);
                    }
                } else {
                    let is_king_side_castling =
                        self.is_king_side_castling(square_idx, destination_idx);

                    let is_queen_side_castling =
                        self.is_queen_side_castling(square_idx, destination_idx);

                    let (can_king_side_castle, can_queen_side_castle) = self.get_castling_rights();

                    // if try to castle without castling rights, skip

                    if is_king_side_castling && !can_king_side_castle {
                        continue;
                    }

                    if is_queen_side_castling && !can_queen_side_castle {
                        continue;
                    }

                    if is_king_side_castling || is_queen_side_castling {
                        let rook_idx: PieceIndex = if is_king_side_castling {
                            destination_idx + 1
                        } else if is_queen_side_castling {
                            destination_idx - 2
                        } else {
                            panic!("failed to castle")
                        };

                        let piece = self.get(rook_idx);
                        let mut is_checked = false;
                        let mut is_blocked = false;
                        let mut idx = square_idx;

                        for _ in 0..2 {
                            if self.is_attacked(idx) {
                                is_checked = true;
                            }

                            if idx != square_idx && self.get(idx) != EMPTY {
                                is_blocked = true;
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
                            && !is_blocked
                        {
                            inner_moves.push(destination_idx);
                        }
                    } else {
                        inner_moves.push(destination_idx);
                    }
                }
            }
        }

        inner_moves
    }

    pub fn generate_knight_moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let mut inner_moves: Vec<PieceIndex> = vec![];

        for delta in KNIGHT_DELTAS {
            // convert to i16 to prevent overflow
            let destination_idx = square_idx as i16 + delta as i16;

            if self.is_on_board(destination_idx as u8) {
                let piece = self.get(destination_idx as PieceIndex);

                if piece != EMPTY && piece != EN_PASSANT_SQUARE && self.is_friendly(piece) {
                    continue;
                }

                inner_moves.push(destination_idx as PieceIndex);
            }
        }

        inner_moves
    }

    pub fn generate_sliding_moves(&mut self, square_idx: PieceIndex, deltas: Vec<i8>) -> Vec<u8> {
        let mut inner_moves: Vec<PieceIndex> = vec![];

        for delta in deltas {
            // convert to i16 to prevent overflow
            let mut destination_idx = square_idx as i16 + delta as i16;

            while self.is_on_board(destination_idx as PieceIndex) {
                let piece = self.get(destination_idx as PieceIndex);

                if piece != EMPTY && piece != EN_PASSANT_SQUARE {
                    // if we encounter a friendly piece, we can't move there
                    if self.is_friendly(piece) {
                        break;
                    } else {
                        // if we encounter an enemy piece, we can capture it but cannot move further
                        inner_moves.push(destination_idx as PieceIndex);
                        break;
                    }
                }

                inner_moves.push(destination_idx as PieceIndex);

                // if the destination square is on the board, we keep searching in that direction until we go off the board
                destination_idx += delta as i16;
            }
        }

        inner_moves
    }

    // TODO if this function causes any bugs down the road, just rewrite it by
    // saving a FEN string of the board everytime a move is made and use the previous FEN when undoing inner_moves
    pub fn undo(&mut self) {
        if let Some(old) = self.history.pop() {
            if old.castle {
                let HistoryEntry {
                    to_idx, from_idx, ..
                } = old;

                // TODO: just hardcode the castle squares
                let king_side = self.is_king_side_castling(from_idx, to_idx);
                let queen_side = self.is_queen_side_castling(from_idx, to_idx);

                if king_side {
                    self.set(ROOK | self.turn, to_idx + 1);
                    self.set(EMPTY, from_idx + 1);
                } else if queen_side {
                    self.set(ROOK | self.turn, to_idx - 2);
                    self.set(EMPTY, from_idx - 1);
                }
            }

            if old.capture {
                if self.turn == WHITE {
                    self.white_captures.pop();
                } else {
                    self.black_captures.pop();
                }
            }

            if old.en_passant_capture {
                let from_idx = old.from_idx;

                let idx = match old.to_idx {
                    i if i < from_idx => old.to_idx + 16,
                    i if i > from_idx => old.to_idx - 16,
                    _ => panic!("could not calculate en passant squares"),
                };

                self.set(EN_PASSANT_SQUARE, idx);

                let piece = old.from_piece;

                if piece == MOVED_PAWN {
                    self.set(MOVED_BLACK_PAWN, old.to_idx + 16);
                } else if piece == MOVED_BLACK_PAWN {
                    self.set(MOVED_PAWN, old.to_idx - 16);
                }
            }

            if old.en_passant_move {
                let from_idx = old.from_idx;

                let idx = match old.to_idx {
                    i if i < from_idx => old.to_idx + 16,
                    i if i > from_idx => old.to_idx - 16,
                    _ => panic!("could not calculate en passant squares"),
                };

                self.set(EMPTY, idx);
            }

            self.can_black_king_side_castle = old.can_black_king_side_castle;
            self.can_black_queen_side_castle = old.can_black_queen_side_castle;
            self.can_white_king_side_castle = old.can_white_king_side_castle;
            self.can_white_queen_side_castle = old.can_white_queen_side_castle;

            self.half_moves = old.half_moves;
            self.full_moves = old.full_moves;

            self.lastest_en_passant_square = old.en_passant_square;
            if let Some(ep_square) = &self.lastest_en_passant_square {
                let ep_idx = BOARD_MAP
                    [self.convert_algebraic_notation_to_index(ep_square.as_str()) as usize];

                if self.get(ep_idx) == EMPTY {
                    self.set(EN_PASSANT_SQUARE, ep_idx);
                }
            }

            // move the pieces back to its original square
            self.set(old.from_piece, old.from_idx);
            self.set(old.to_piece, old.to_idx);
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
        let en_passant_square = if let Some(sq) = self.lastest_en_passant_square.clone() {
            sq
        } else {
            "-".to_string()
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

                let piece_type = self.get_type(piece);

                if self.get_color(piece) == WHITE {
                    match piece_type {
                        PAWN => fen.push_str("P"),
                        ROOK => fen.push_str("R"),
                        KNIGHT => fen.push_str("N"),
                        BISHOP => fen.push_str("B"),
                        QUEEN => fen.push_str("Q"),
                        KING => fen.push_str("K"),
                        _ => panic!("error generating FEN"),
                    }
                } else {
                    match piece_type {
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
        let fen_parts: Vec<&str> = fen.split(" ").collect();

        let ranks: Vec<&str> = fen_parts[0].split("/").collect();

        let mut idx: usize = 0;
        for rank in ranks {
            for piece in rank.chars() {
                match piece {
                    'p' => {
                        let rank = 8 - ((BOARD_MAP[idx] >> 4) + 1) + 1;

                        if rank != 7 {
                            self.set(MOVED_BLACK_PAWN, BOARD_MAP[idx])
                        } else {
                            self.set(BLACK_PAWN, BOARD_MAP[idx])
                        }
                    }
                    'r' => self.set(BLACK_ROOK, BOARD_MAP[idx]),
                    'n' => self.set(BLACK_KNIGHT, BOARD_MAP[idx]),
                    'b' => self.set(BLACK_BISHOP, BOARD_MAP[idx]),
                    'q' => self.set(BLACK_QUEEN, BOARD_MAP[idx]),
                    'k' => self.set(BLACK_KING, BOARD_MAP[idx]),
                    'P' => {
                        let rank = 8 - ((BOARD_MAP[idx] >> 4) + 1) + 1;

                        if rank != 2 {
                            self.set(MOVED_PAWN, BOARD_MAP[idx])
                        } else {
                            self.set(PAWN, BOARD_MAP[idx])
                        }
                    }
                    'R' => self.set(ROOK, BOARD_MAP[idx]),
                    'N' => self.set(KNIGHT, BOARD_MAP[idx]),
                    'B' => self.set(BISHOP, BOARD_MAP[idx]),
                    'Q' => self.set(QUEEN, BOARD_MAP[idx]),
                    'K' => self.set(KING, BOARD_MAP[idx]),
                    '1'..='8' => idx += piece.to_digit(10).unwrap() as usize - 1,
                    _ => panic!("can't load fen pieces"),
                }

                idx += 1;
            }
        }

        // set turn
        match fen_parts[1] {
            "w" => self.set_turn(WHITE),
            "b" => self.set_turn(BLACK),
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
            "-" => {
                self.lastest_en_passant_square = None;
            }
            _ => {
                self.lastest_en_passant_square = Some(square.to_string());
                let idx = self.convert_algebraic_notation_to_index(square) as usize;

                self.set(EN_PASSANT_SQUARE, BOARD_MAP[idx]);
            }
        }

        self.half_moves = fen_parts[4].parse().unwrap();
        self.full_moves = fen_parts[5].parse().unwrap();

        *self
            .unique_positions
            .entry(fen_parts[0].to_string())
            .or_insert(0) += 1;
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
                let inner_moves = self.inner_moves(idx);

                if inner_moves.len() > 0 {
                    no_legal_moves = false;
                }
            }
        }

        self.in_check() && no_legal_moves
    }

    pub fn is_draw(&mut self) -> bool {
        self.is_stalemate()
            || self.is_threefold_repetition()
            || self.is_50_moves_rule()
            || self.is_insufficient_materials()
    }

    /// stalemate happens when a player has no legal inner_moves and is not in check
    pub fn is_stalemate(&mut self) -> bool {
        let mut no_legal_moves = true;

        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);

            if (piece != EMPTY || piece != EN_PASSANT_SQUARE) && self.is_friendly(piece) {
                let inner_moves = self.inner_moves(idx);

                if inner_moves.len() > 0 {
                    no_legal_moves = false;
                }
            }
        }

        !self.in_check() && no_legal_moves
    }

    // convert every move to FEN, store it as a unique key in a hashmap
    // if the value >= 3, then it is threefold repetition
    pub fn is_threefold_repetition(&mut self) -> bool {
        let fen = self.get_fen();
        let position = fen.split(" ").collect::<Vec<&str>>()[0];

        if let Some(count) = self.unique_positions.get(position) {
            *count >= 3
        } else {
            false
        }
    }

    pub fn is_50_moves_rule(&mut self) -> bool {
        self.half_moves >= 100
    }

    /*
        If both sides have any one of the following, and there are no pawns or other pieces on the board:

        A lone king
        a king and bishop (of same colors)
        a king and knight

        a king and two knights vs a lone king = draw

        accoring to https://support.chess.com/article/128-what-does-insufficient-mating-material-mean
    */
    pub fn is_insufficient_materials(&mut self) -> bool {
        let mut friendly_knights = 0;
        let mut friendly_bishops = 0;
        let mut friendly_dark_bishops = 0;
        let mut friendly_light_bishops = 0;
        let mut enemy_dark_bishops = 0;
        let mut enemy_light_bishops = 0;
        let mut enemy_knights = 0;
        let mut enemy_bishops = 0;

        for idx in 0..BOARD_SIZE {
            if !self.is_on_board(idx) {
                continue;
            }

            let piece = self.get(idx);
            let file = (idx & 7) + 1;
            let rank = 8 - ((idx >> 4) + 1) + 1;

            // if rank is odd and file is odd = dark
            // if rank is even and file is even = dark

            // if rank is even and file is odd = light
            // if rank is odd and file is even = light

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
                        // dark square bishop
                        if (rank % 2 != 0 && file % 2 != 0) || (rank % 2 == 0 && file % 2 == 0) {
                            friendly_dark_bishops += 1;

                            if enemy_light_bishops >= 1 {
                                return false;
                            }
                        }

                        // light square bishop
                        if (rank % 2 == 0 && file % 2 != 0) || (rank % 2 != 0 && file % 2 == 0) {
                            friendly_light_bishops += 1;

                            if enemy_dark_bishops >= 1 {
                                return false;
                            }
                        }

                        friendly_bishops += 1;
                    }
                } else {
                    if piece_without_color == KNIGHT {
                        enemy_knights += 1;
                    }

                    if piece_without_color == BISHOP {
                        // dark square bishop

                        if (rank % 2 != 0 && file % 2 != 0) || (rank % 2 == 0 && file % 2 == 0) {
                            enemy_dark_bishops += 1;

                            if friendly_light_bishops >= 1 {
                                return false;
                            }
                        }

                        // light square bishop
                        if (rank % 2 == 0 && file % 2 != 0) || (rank % 2 != 0 && file % 2 == 0) {
                            enemy_light_bishops += 1;

                            if friendly_dark_bishops >= 1 {
                                return false;
                            }
                        }

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
            if friendly_knights >= 1 && enemy_knights >= 1 {
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

            if is_attacked {
                break;
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
                let piece_type = self.get_type(piece);

                // check if that piece can attack from that particular square
                if (piece_type & attack_bits_mask) == piece_type {
                    if piece_type == KNIGHT {
                        return true;
                    } else if piece_type == PAWN {
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
                                break;
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

        let file = FILES
            .iter()
            .position(|f| f.eq(&file.to_string().as_str()))
            .unwrap() as u8;

        // we minus rank from 7 because the board is reversed (upside down)
        // so for example, for "e7", the rank 7 is rank 1 on our board
        (8 * (7 - rank) + file) as u8
    }

    pub fn convert_index_algebraic_notation(&self, index: u8) -> String {
        let file = index & 7;
        let rank = 8 - ((index >> 4) + 1) + 1;

        let file_letter = FILES[file as usize];

        let mut notation = String::new();

        notation.push_str(file_letter);
        notation.push_str(rank.to_string().as_str());

        notation
    }

    fn reset_half_moves(&mut self) {
        self.half_moves = 0;
    }

    fn change_turn(&mut self) {
        if self.turn == WHITE {
            self.last_turn = WHITE;
            self.set_turn(BLACK);
        } else {
            self.last_turn = BLACK;
            self.set_turn(WHITE);
        }
    }

    fn clear_latest_en_passant_square(&mut self) {
        if let Some(sq) = &self.lastest_en_passant_square {
            let idx = BOARD_MAP[self.convert_algebraic_notation_to_index(sq.as_str()) as usize];

            let piece = self.get(idx);

            if piece == EN_PASSANT_SQUARE {
                self.set(EMPTY, idx);
                self.lastest_en_passant_square = None;
            }
        }
    }

    fn get_type(&self, piece: PieceType) -> PieceType {
        self.remove_mask(self.remove_color(piece), MOVED_MASK)
    }

    fn get_color(&self, piece: PieceType) -> PieceType {
        piece & COLOR_MASK
    }

    fn remove_color(&self, piece: PieceType) -> PieceType {
        self.remove_mask(piece, COLOR_MASK)
    }

    /// (white kingside, white queenside, blac kingside, black queenside)
    pub fn get_castling_rights_tests(&self) -> (bool, bool, bool, bool) {
        (
            self.can_white_king_side_castle,
            self.can_white_queen_side_castle,
            self.can_black_king_side_castle,
            self.can_black_queen_side_castle,
        )
    }

    pub fn get_castling_rights(&self) -> (bool, bool) {
        if self.turn == WHITE {
            (
                self.can_white_king_side_castle,
                self.can_white_queen_side_castle,
            )
        } else {
            (
                self.can_black_king_side_castle,
                self.can_black_queen_side_castle,
            )
        }
    }

    fn update_castling_rights(&mut self) {
        if self.kings.white != 116 {
            self.can_white_king_side_castle = false;
            self.can_white_queen_side_castle = false;
        }

        if self.kings.black != 4 {
            self.can_black_king_side_castle = false;
            self.can_black_queen_side_castle = false;
        }
        // 119 = h1
        // 112 = a1
        let right_white_rook_idx = 119;
        let left_white_rook_idx = 112;

        // 7 = h8
        // 0 = a8
        let right_black_rook_idx = 7;
        let left_black_rook_idx = 0;

        // if it isn't an *unmoved* rook, then we can't castle

        if self.get(right_white_rook_idx) != ROOK {
            self.can_white_king_side_castle = false;
        }

        if self.get(left_white_rook_idx) != ROOK {
            self.can_white_queen_side_castle = false;
        }

        if self.get(right_black_rook_idx) != BLACK_ROOK {
            self.can_black_king_side_castle = false;
        }

        if self.get(left_black_rook_idx) != BLACK_ROOK {
            self.can_black_queen_side_castle = false;
        }
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

// TODO: sort the inner_moves array in tests to ensure they match

#[cfg(test)]
mod bishop {
    use super::*;

    #[test]
    fn bishop_can_move_freely_if_king_is_not_checked() {
        let mut chess = Chess::new();

        chess.set(BISHOP, 52);
        chess.set(KING, 55);
        chess.set(BISHOP | BLACK, 86);

        let inner_moves = chess.inner_moves(52);
        let correct_moves = [69, 86, 67, 82, 97, 112, 35, 18, 1, 37, 22, 7];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 0);
        chess.set(KING, 1);
        chess.set(BISHOP | BLACK, 7);

        let inner_moves = chess.inner_moves(0);
        let correct_moves = [17, 34, 51, 68, 85, 102, 119];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 103);
        chess.set(KING | BLACK, 81);
        chess.set(BISHOP | BLACK, 36);

        let inner_moves = chess.inner_moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 103);
        chess.set(KING | BLACK, 81);
        chess.set(BISHOP | BLACK, 36);

        let inner_moves = chess.inner_moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 0);
        chess.set(KING | BLACK, 7);
        chess.set(BISHOP | BLACK, 112);

        let inner_moves = chess.inner_moves(112);
        let correct_moves = [97, 82, 67, 52, 37, 22];

        assert!(inner_moves.iter().eq(correct_moves.iter()));
    }

    #[test]
    fn bishop_can_not_move_because_king_is_checked() {
        let mut chess = Chess::new();

        chess.set(BISHOP, 117);
        chess.set(KING, 112);
        chess.set(BLACK_BISHOP, 97);

        let inner_moves = chess.inner_moves(117);

        assert!(inner_moves.len() == 0);

        chess.clear();

        chess.set(BISHOP, 119);
        chess.set(KING, 7);
        chess.set(BISHOP | BLACK, 22);

        let inner_moves = chess.inner_moves(119 as u8);
        assert!(inner_moves.len() == 0);

        chess.clear();

        chess.set(BISHOP, 67);
        chess.set(KING, 0);
        chess.set(BISHOP | BLACK, 17);

        let inner_moves = chess.inner_moves(67 as u8);
        assert!(inner_moves.len() == 0);

        chess.clear();

        //======= BLACK =======
        chess.set_turn(BLACK);

        chess.set(BISHOP, 0);
        chess.set(KING | BLACK, 17);
        chess.set(BISHOP | BLACK, 51);

        let inner_moves = chess.inner_moves(51);
        assert!(inner_moves.len() == 0);

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 67);
        chess.set(KING | BLACK, 112);
        chess.set(BISHOP | BLACK, 100);

        let inner_moves = chess.inner_moves(100);
        assert!(inner_moves.len() == 0);

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 118);
        chess.set(KING | BLACK, 16);
        chess.set(BISHOP | BLACK, 21);

        let inner_moves = chess.inner_moves(21);
        assert!(inner_moves.len() == 0);
    }

    #[test]
    fn bishop_can_take_enemy_piece_to_stop_check() {
        let mut chess = Chess::new();

        chess.set(BISHOP, 2);
        chess.set(KING, 0);
        chess.set(BISHOP | BLACK, 17);

        let inner_moves = chess.inner_moves(2 as u8);
        let correct_moves = [17];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 99);
        chess.set(KING, 37);
        chess.set(BISHOP | BLACK, 54);

        let inner_moves = chess.inner_moves(99 as u8);
        let correct_moves = [54];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        // ====== BLACK =====

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 32);
        chess.set(KING | BLACK, 66);
        chess.set(BISHOP | BLACK, 17);

        let inner_moves = chess.inner_moves(17);
        let correct_moves = [32];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 67);
        chess.set(KING | BLACK, 7);
        chess.set(BISHOP | BLACK, 118);

        let inner_moves = chess.inner_moves(118);
        let correct_moves = [67];

        assert!(inner_moves.iter().eq(correct_moves.iter()));
    }

    #[test]
    fn bishop_can_move_freely_if_king_is_shielded_from_check() {
        let mut chess = Chess::new();
        chess.set(BISHOP, 51);
        chess.set(KING, 7);
        chess.set(PAWN, 37);
        chess.set(BISHOP | BLACK, 67);

        let inner_moves = chess.inner_moves(51);
        let correct_moves = [68, 85, 102, 119, 66, 81, 96, 34, 17, 0, 36, 21, 6];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 118);
        chess.set(KING, 119);
        chess.set(PAWN, 102);
        chess.set(BISHOP | BLACK, 85);

        let inner_moves = chess.inner_moves(118);
        let correct_moves = [101, 84, 67, 50, 33, 16, 103];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 66);
        chess.set(KING, 17);
        chess.set(PAWN, 34);
        chess.set(BISHOP | BLACK, 51);

        let inner_moves = chess.inner_moves(66);
        let correct_moves = [83, 100, 117, 81, 96, 49, 32, 51];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        // ==== BLACK ====
        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 64);
        chess.set(KING | BLACK, 4);
        chess.set(PAWN | BLACK, 19);
        chess.set(BISHOP | BLACK, 84);

        let inner_moves = chess.inner_moves(84);
        let correct_moves = [101, 118, 99, 114, 67, 50, 33, 16, 69, 54, 39];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 53);
        chess.set(KING | BLACK, 87);
        chess.set(PAWN | BLACK, 70);
        chess.set(BISHOP | BLACK, 98);

        let inner_moves = chess.inner_moves(98);
        let correct_moves = [115, 113, 81, 64, 83, 68, 53];

        assert!(inner_moves.iter().eq(correct_moves.iter()));
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
        let inner_moves = chess.inner_moves(98);
        let correct_moves = [114];

        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set_turn(BLACK);
        chess.set(Piece::BLACK_PAWN, 2);
        let inner_moves = chess.inner_moves(2);
        let correct_moves = [18, 34];
        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::PAWN, 34);
        let inner_moves = chess.inner_moves(34);
        let correct_moves = [18, 2];
        assert!(inner_moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::PAWN, 23);
        let inner_moves = chess.inner_moves(23);
        let correct_moves = [7];
        assert!(inner_moves.iter().eq(correct_moves.iter()));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lol() {}
}
