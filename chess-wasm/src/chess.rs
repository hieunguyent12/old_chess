use crate::chess::Piece::*;

const BOARD_SIZE: u8 = 128;
const COLOR_MASK: u8 = 128; // 10000000
const MOVED_MASK: u8 = 2; // 00000010

// use | to make a piece black or white
// use & to check if a piece is black or white
pub const WHITE: u8 = 0;
pub const BLACK: u8 = 128;

#[allow(non_snake_case)]
pub mod Piece {
    use crate::chess::BLACK;

    pub type PieceIndex = u8;
    pub type PieceType = u8;

    pub const EMPTY: PieceType = 0; //         00000000
    pub const MOVED_PAWN: PieceType = 3; //    00000011
    pub const MOVED_BLACK_PAWN: PieceType = MOVED_PAWN | BLACK; // 10000011

    pub const PAWN: PieceType = 1; //          00000001   Moved pawn: 00000011
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

// why does it have 239 items?
// because of how the indexes of the squares on the real board are laid out due to the fact that
// we have to use some indexes in between to represent the dummy board
/*
    q & b = 40
    q & r = 34
    q & b & k = 56

    queen, bishop, king = 56

    queen, bishop, white pawn, king = 57  = 00111001
    queen, bishop, black pawn, king = 185

    00100000
    00001000
    10000001
    00010000
    10111001


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
     1,  1,  1,  1,  1,  1,  1,  0, -1, -1,  -1,-1, -1, -1, -1, 0,
     0,  0,  0,  0,  0,  0, 15, 16, 17,  0,  0,  0,  0,  0,  0, 0,
     0,  0,  0,  0,  0, 15,  0, 16,  0, 17,  0,  0,  0,  0,  0, 0,
     0,  0,  0,  0, 15,  0,  0, 16,  0,  0, 17,  0,  0,  0,  0, 0,
     0,  0,  0, 15,  0,  0,  0, 16,  0,  0,  0, 17,  0,  0,  0, 0,
     0,  0, 15,  0,  0,  0,  0, 16,  0,  0,  0,  0, 17,  0,  0, 0,
     0, 15,  0,  0,  0,  0,  0, 16,  0,  0,  0,  0,  0, 17,  0, 0,
    15,  0,  0,  0,  0,  0,  0, 16,  0,  0,  0,  0,  0,  0, 17
 ];

struct History {
    from_idx: PieceIndex,
    to_idx: PieceIndex,
    from_piece: PieceType,
    to_piece: PieceType,
}

struct King {
    white: PieceIndex,
    black: PieceIndex,
}

pub struct Chess {
    board: [PieceType; BOARD_SIZE as usize],

    /// 0 = white, 128 = black
    turn: u8,

    history: Vec<History>,

    /// the kings' indices on the board
    kings: King,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            board: [0; BOARD_SIZE as usize],
            turn: WHITE,
            history: vec![],
            kings: King { white: 1, black: 2 },
        }
    }

    pub fn move_piece(&mut self, from_idx: PieceIndex, to_idx: PieceIndex) {
        // TODO: move the piece as long as the king is not in checked
        if self.is_on_board(to_idx)
        /* && self.get(to_idx) == 0 */
        {
            let piece = self.get(from_idx);
            let to_piece = self.get(to_idx);

            self.set(piece, to_idx);
            self.set(Piece::EMPTY, from_idx);

            self.history.push(History {
                from_idx,
                to_idx,
                from_piece: piece,
                to_piece,
            })
        } else {
            // panic!("illegal move!")
        }
    }

    /// Return the piece on the square
    pub fn get(&self, square_idx: PieceIndex) -> PieceIndex {
        self.board[square_idx as usize]
    }

    /// Put a piece on a square
    pub fn set(&mut self, piece: PieceType, square_idx: PieceIndex) {
        if piece == KING {
            self.kings.white = square_idx;
        }

        if piece == KING | BLACK {
            self.kings.black = square_idx;
        }

        self.board[square_idx as usize] = piece;
    }

    pub fn moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let piece = self.get(square_idx);

        let moves = match self.remove_color(piece) {
            PAWN => self.generate_pawn_moves(square_idx),
            MOVED_PAWN => self.generate_pawn_moves(square_idx),
            BISHOP => self.generate_diagonal_sliding_moves(square_idx),
            _ => vec![],
        };

        let mut legalMoves: Vec<u8> = vec![];

        for idx in moves {
            let destination_idx = idx;
            // play the move
            self.move_piece(square_idx, destination_idx as u8);

            let mut in_check = false;

            // TODO: loop through all the enemy pieces? - maybe do this AFTER we generate all the moves so we only loop once?
            for idx in 0..BOARD_SIZE {
                let piece = self.get(idx);

                // if piece is friendly, skip
                if self.is_friendly(piece) || piece == EMPTY {
                    continue;
                } else {
                    let king_idx = match self.turn {
                        WHITE => self.kings.white,
                        BLACK => self.kings.black,
                        _ => panic!("Turn cannot be determined when checking if king is attacked"),
                    };

                    if self.is_attacked(king_idx, idx) {
                        println!("King is attacked!");
                        println!(
                            "Enemy: {} - Destination: {} - From: {}",
                            idx, destination_idx, square_idx
                        );
                        in_check = true;
                        break;
                    }
                }
            }

            // revert the move
            self.undo();

            if !in_check {
                legalMoves.push(destination_idx);
            }
        }

        legalMoves
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

        for delta in deltas {
            if delta == 0 {
                continue;
            }

            let destination_idx = square_idx as i16 + delta as i16;
            let destination_idx = destination_idx as u8;

            if self.is_on_board(destination_idx) {
                moves.push(destination_idx);
            }
        }

        moves
    }

    pub fn generate_diagonal_sliding_moves(&mut self, square_idx: PieceIndex) -> Vec<u8> {
        let mut moves: Vec<PieceIndex> = vec![];

        for delta in BISHOP_DELTAS {
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

    pub fn undo(&mut self) {
        if let Some(player_move) = self.history.pop() {
            // move the pieces back to its original square
            self.set(player_move.from_piece, player_move.from_idx);
            self.set(player_move.to_piece, player_move.to_idx);
        }
    }

    pub fn is_attacked(&self, defender_idx: PieceIndex, attacker_idx: PieceIndex) -> bool {
        let diff = (defender_idx as i16 - attacker_idx as i16) + 119;

        let attack_bits_mask = ATTACKS[diff as usize];

        if attack_bits_mask == 0 {
            return false;
        } else {
            // although the king can be attacked from a particular square, we also
            // have to take into account if that piece can attack from there
            let piece = self.get(attacker_idx);

            // remove the color mask
            let piece_without_color = (piece | COLOR_MASK) ^ COLOR_MASK;

            // check if that piece can attack from that particular square
            if (piece_without_color & attack_bits_mask) == piece_without_color {
                if piece_without_color == KNIGHT {
                    return true;
                } else if piece_without_color == PAWN {
                    let pawn = piece;
                    let black_pawn = PAWN | BLACK;
                    // if it is black, it can only attack down
                    if pawn == black_pawn {
                        if pawn & attack_bits_mask == black_pawn {
                            return true;
                        }
                    } else {
                        // if the pawn is white, it can only attack up
                        if pawn & attack_bits_mask == PAWN {
                            return true;
                        }
                    }

                    return false;
                } else {
                    // check if there is a piece standing in the way of the attack
                    let delta = DELTAS[diff as usize];

                    let mut destination_idx = attacker_idx as i16 + delta as i16;

                    while self.is_on_board(destination_idx as PieceIndex) {
                        let piece = self.get(destination_idx as PieceIndex);

                        if piece != EMPTY {
                            // if the piece is the king, then return true because the king is attacked
                            if piece & KING == KING && self.is_friendly(piece) {
                                return true;
                            }
                            // if its the king, then another piece is blocking the check
                            return false;
                        }

                        destination_idx += delta as i16;
                    }

                    return true;
                }
            } else {
                return false;
            }
        }
    }

    pub fn current_turn(&self) {
        match self.turn {
            0 => {}
            1 => {}
            _ => panic!("invalid turn"),
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

    fn remove_color(&self, piece: PieceType) -> PieceType {
        (piece | COLOR_MASK) ^ COLOR_MASK
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

        let moves = chess.generate_diagonal_sliding_moves(52);
        let correct_moves = [69, 86, 67, 82, 97, 112, 35, 18, 1, 37, 22, 7];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(BISHOP, 0);
        chess.set(KING, 1);
        chess.set(BISHOP | BLACK, 7);

        let moves = chess.generate_diagonal_sliding_moves(0);
        let correct_moves = [17, 34, 51, 68, 85, 102, 119];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 103);
        chess.set(KING | BLACK, 81);
        chess.set(BISHOP | BLACK, 36);

        let moves = chess.generate_diagonal_sliding_moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 103);
        chess.set(KING | BLACK, 81);
        chess.set(BISHOP | BLACK, 36);

        let moves = chess.generate_diagonal_sliding_moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set(BISHOP, 0);
        chess.set(KING | BLACK, 7);
        chess.set(BISHOP | BLACK, 112);

        let moves = chess.generate_diagonal_sliding_moves(112);
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

        chess.set(Piece::BLACK_PAWN, 98);
        let moves = chess.generate_pawn_moves(98);
        let correct_moves = [114, 115, 113];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::BLACK_PAWN, 2);
        let moves = chess.generate_pawn_moves(2);
        let correct_moves = [18, 34, 19, 17];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::PAWN, 34);
        let moves = chess.generate_pawn_moves(34);
        let correct_moves = [18, 2, 17, 19];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set(Piece::PAWN, 23);
        let moves = chess.generate_pawn_moves(23);
        let correct_moves = [7, 6];
        assert!(moves.iter().eq(correct_moves.iter()));
    }
}