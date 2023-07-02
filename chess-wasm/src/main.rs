const EMPTY: u8 = 0; //         00000000
const PAWN: u8 = 1; //          00000001
const ROOK: i8 = 2; //          00000010
const KNIGHT: u8 = 4; //        00000100
const BISHOP: u8 = 8; //        00001000
const KING: u8 = 16; //         00010000
const QUEEN: i8 = 32; //        00100000

// const PIECE_MASK: i8 = 63; //   00111111
const COLOR_MASK: u8 = 128; // 10000000

const BOARD_SIZE: u8 = 128;

// use | to make a piece black or white
// use & to check if a piece is black or white
const WHITE: u8 = 0;
const BLACK: u8 = 128;

const BISHOP_DELTAS: [i8; 4] = [17, 15, -17, -15];

// why does it have 239 items?
// because of how the indexes of the squares on the real board are laid out due to the fact that
// we have to use some indexes in between to represent the dummy board
/*
    q & b = 40
    q & r = 34
    q & b & k = 56

    queen, bishop, king =
    00100000
    00001000
    00010000
    00111000

    queen, bishop, pawn, king = 57  = 00111001

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
   0, 0, 0, 0, 0, 4, 57, 50, 57, 4, 0, 0, 0, 0, 0, 0, // But the piece isn't always in the middle? We can "move" it to the middle by adding 119
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

/*
  const attacks2 = [
   5,0,0,0,0,0,0,2,0,0,0,0,0,0,5,0,
   0,5,0,0,0,0,0,2,0,0,0,0,0,5,0,0
   0,0,5,0,0,0,0,2,0,0,0,0,5,0,0,0,
   0,0,0,5,0,0,0,2,0,0,0,5,0,0,0,0,
   0,0,0,0,5,0,0,2,0,0,5,0,0,0,0,0,
   0,0,0,0,0,5,6,2,6,5,0,0,0,0,0,0,
   0,0,0,0,0,6,4,1,4,6,0,0,0,0,0,0,
   2,2,2,2,2,2,1,0,1,2,2,2,2,2,2,0,
   0,0,0,0,0,6,3,1,3,6,0,0,0,0,0,0,
   0,0,0,0,0,5,6,2,6,5,0,0,0,0,0,0,
   0,0,0,0,5,0,0,2,0,0,5,0,0,0,0,0,
   0,0,0,5,0,0,0,2,0,0,0,5,0,0,0,0,
   0,0,5,0,0,0,0,2,0,0,0,0,5,0,0,0,
   0,5,0,0,0,0,0,2,0,0,0,0,0,5,0,0,
   5,0,0,0,0,0,0,2,0,0,0,0,0,0,5,0,
];

 */

struct History {
    from_idx: u8,
    to_idx: u8,
    from_piece: u8,
    to_piece: u8,
}

struct King {
    white: u8,
    black: u8,
}

struct Chess {
    board: [u8; BOARD_SIZE as usize],

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

    pub fn move_piece(&mut self, from_idx: u8, to_idx: u8) {
        // TODO: move the piece as long as the king is not in checked
        if self.is_on_board(to_idx)
        /* && self.get_piece_at(to_idx) == 0 */
        {
            let piece = self.get_piece_at(from_idx);
            let to_piece = self.get_piece_at(to_idx);

            self.set_piece_at(piece, to_idx);
            self.set_piece_at(EMPTY, from_idx);

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

    pub fn get_piece_at(&self, square_idx: u8) -> u8 {
        self.board[square_idx as usize]
    }

    pub fn set_piece_at(&mut self, piece: u8, square_idx: u8) {
        if piece == KING {
            self.kings.white = square_idx;
        }

        if piece == KING | BLACK {
            self.kings.black = square_idx;
        }

        self.board[square_idx as usize] = piece;
    }

    pub fn generate_diagonal_sliding_moves(&mut self, square_idx: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = vec![];

        for delta in BISHOP_DELTAS {
            // convert to i16 to prevent overflow
            let mut destination_idx = square_idx as i16 + delta as i16;

            while self.is_on_board(destination_idx as u8) {
                let piece = self.get_piece_at(destination_idx as u8);

                if piece != EMPTY {
                    // if we encounter a friendly piece, we can't move there
                    if self.is_friendly(piece) {
                        break;
                    } else {
                        // if we encounter an enemy piece, we can capture it but cannot move further
                        moves.push(destination_idx as u8);
                        break;
                    }
                }

                // play the move
                self.move_piece(square_idx, destination_idx as u8);

                let mut is_king_checked = false;

                // TODO: loop through all the enemy pieces? - maybe do this AFTER we generate all the moves so we only loop once?
                for idx in 0..BOARD_SIZE {
                    let piece = self.get_piece_at(idx);

                    // if piece is friendly, skip
                    if self.is_friendly(piece) || piece == EMPTY {
                        continue;
                    } else {
                        let king_idx = match self.turn {
                            WHITE => self.kings.white,
                            BLACK => self.kings.black,
                            _ => panic!(
                                "Turn cannot be determined when checking if king is attacked"
                            ),
                        };

                        if self.is_attacked(king_idx, idx) {
                            println!("King is attacked!");
                            println!(
                                "Enemy: {} - Destination: {} - From: {}",
                                idx, destination_idx, square_idx
                            );
                            is_king_checked = true;

                            break;
                        }
                    }
                }

                // revert the move
                self.undo();

                if !is_king_checked {
                    moves.push(destination_idx as u8);
                }

                // if the destination square is on the board, we keep searching in that direction until we go off the board
                destination_idx += delta as i16;
            }
        }

        moves
    }

    pub fn undo(&mut self) {
        if let Some(player_move) = self.history.pop() {
            // move the pieces back to its original square
            self.set_piece_at(player_move.from_piece, player_move.from_idx);
            self.set_piece_at(player_move.to_piece, player_move.to_idx);
        }
    }

    pub fn is_attacked(&self, defender_idx: u8, attacker_idx: u8) -> bool {
        let diff = (defender_idx as i16 - attacker_idx as i16) + 119;

        let attack_bits_mask = ATTACKS[diff as usize];

        if attack_bits_mask == 0 {
            return false;
        } else {
            // although the king can be attacked from a particular square, we also
            // have to take into account if that piece can attack from there
            let piece = self.get_piece_at(attacker_idx);

            // remove the color mask
            let piece_without_color = (piece | COLOR_MASK) ^ COLOR_MASK;

            // check if that piece can attack from that particular square
            if (piece_without_color & attack_bits_mask) == piece_without_color {
                if piece_without_color == KNIGHT || piece_without_color == PAWN {
                    return true;
                } else {
                    // check if there is a piece standing in the way of the attack
                    let delta = DELTAS[diff as usize];

                    let mut destination_idx = attacker_idx as i16 + delta as i16;

                    while self.is_on_board(destination_idx as u8) {
                        let piece = self.get_piece_at(destination_idx as u8);

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

    /// a piece is friendly if its color matches the current player's turn color
    fn is_friendly(&self, piece: u8) -> bool {
        piece & COLOR_MASK == self.turn
    }

    fn is_on_board(&self, square_idx: u8) -> bool {
        (square_idx) & 0x88 == 0
    }
}

#[cfg(test)]
mod bishop {
    use super::*;

    #[test]
    fn bishop_can_move_freely_if_king_is_not_checked() {
        let mut chess = Chess::new();

        chess.set_piece_at(BISHOP, 52);
        chess.set_piece_at(KING, 55);
        chess.set_piece_at(BISHOP | BLACK, 86);

        let moves = chess.generate_diagonal_sliding_moves(52);
        let correct_moves = [69, 86, 67, 82, 97, 112, 35, 18, 1, 37, 22, 7];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set_piece_at(BISHOP, 0);
        chess.set_piece_at(KING, 1);
        chess.set_piece_at(BISHOP | BLACK, 7);

        let moves = chess.generate_diagonal_sliding_moves(0);
        let correct_moves = [17, 34, 51, 68, 85, 102, 119];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 103);
        chess.set_piece_at(KING | BLACK, 81);
        chess.set_piece_at(BISHOP | BLACK, 36);

        let moves = chess.generate_diagonal_sliding_moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 103);
        chess.set_piece_at(KING | BLACK, 81);
        chess.set_piece_at(BISHOP | BLACK, 36);

        let moves = chess.generate_diagonal_sliding_moves(36);
        let correct_moves = [53, 70, 87, 51, 66, 19, 2, 21, 6];
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 0);
        chess.set_piece_at(KING | BLACK, 7);
        chess.set_piece_at(BISHOP | BLACK, 112);

        let moves = chess.generate_diagonal_sliding_moves(112);
        let correct_moves = [97, 82, 67, 52, 37, 22];

        assert!(moves.iter().eq(correct_moves.iter()));
    }

    #[test]
    fn bishop_can_not_move_because_king_is_checked() {
        let mut chess = Chess::new();

        chess.set_piece_at(BISHOP, 117);
        chess.set_piece_at(KING, 112);
        chess.set_piece_at(BISHOP | BLACK, 97);

        let moves = chess.generate_diagonal_sliding_moves(117 as u8);

        assert!(moves.len() == 0);

        chess.clear();

        chess.set_piece_at(BISHOP, 119);
        chess.set_piece_at(KING, 7);
        chess.set_piece_at(BISHOP | BLACK, 22);

        let moves = chess.generate_diagonal_sliding_moves(119 as u8);
        assert!(moves.len() == 0);

        chess.clear();

        chess.set_piece_at(BISHOP, 67);
        chess.set_piece_at(KING, 0);
        chess.set_piece_at(BISHOP | BLACK, 17);

        let moves = chess.generate_diagonal_sliding_moves(67 as u8);
        assert!(moves.len() == 0);

        chess.clear();

        //======= BLACK =======
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 0);
        chess.set_piece_at(KING | BLACK, 17);
        chess.set_piece_at(BISHOP | BLACK, 51);

        let moves = chess.generate_diagonal_sliding_moves(51);
        assert!(moves.len() == 0);

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 67);
        chess.set_piece_at(KING | BLACK, 112);
        chess.set_piece_at(BISHOP | BLACK, 100);

        let moves = chess.generate_diagonal_sliding_moves(100);
        assert!(moves.len() == 0);

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 118);
        chess.set_piece_at(KING | BLACK, 16);
        chess.set_piece_at(BISHOP | BLACK, 21);

        let moves = chess.generate_diagonal_sliding_moves(21);
        assert!(moves.len() == 0);
    }

    #[test]
    fn bishop_can_take_enemy_piece_to_stop_check() {
        let mut chess = Chess::new();

        chess.set_piece_at(BISHOP, 2);
        chess.set_piece_at(KING, 0);
        chess.set_piece_at(BISHOP | BLACK, 17);

        let moves = chess.generate_diagonal_sliding_moves(2 as u8);
        let correct_moves = [17];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set_piece_at(BISHOP, 99);
        chess.set_piece_at(KING, 37);
        chess.set_piece_at(BISHOP | BLACK, 54);

        let moves = chess.generate_diagonal_sliding_moves(99 as u8);
        let correct_moves = [54];

        assert!(moves.iter().eq(correct_moves.iter()));

        // ====== BLACK =====

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 32);
        chess.set_piece_at(KING | BLACK, 66);
        chess.set_piece_at(BISHOP | BLACK, 17);

        let moves = chess.generate_diagonal_sliding_moves(17);
        let correct_moves = [32];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 67);
        chess.set_piece_at(KING | BLACK, 7);
        chess.set_piece_at(BISHOP | BLACK, 118);

        let moves = chess.generate_diagonal_sliding_moves(118);
        let correct_moves = [67];

        assert!(moves.iter().eq(correct_moves.iter()));
    }

    #[test]
    fn bishop_can_move_freely_if_king_is_shielded_from_check() {
        let mut chess = Chess::new();
        chess.set_piece_at(BISHOP, 51);
        chess.set_piece_at(KING, 7);
        chess.set_piece_at(PAWN, 37);
        chess.set_piece_at(BISHOP | BLACK, 67);

        let moves = chess.generate_diagonal_sliding_moves(51);
        let correct_moves = [68, 85, 102, 119, 66, 81, 96, 34, 17, 0, 36, 21, 6];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set_piece_at(BISHOP, 118);
        chess.set_piece_at(KING, 119);
        chess.set_piece_at(PAWN, 102);
        chess.set_piece_at(BISHOP | BLACK, 85);

        let moves = chess.generate_diagonal_sliding_moves(118);
        let correct_moves = [101, 84, 67, 50, 33, 16, 103];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        chess.set_piece_at(BISHOP, 66);
        chess.set_piece_at(KING, 17);
        chess.set_piece_at(PAWN, 34);
        chess.set_piece_at(BISHOP | BLACK, 51);

        let moves = chess.generate_diagonal_sliding_moves(66);
        let correct_moves = [83, 100, 117, 81, 96, 49, 32, 51];

        assert!(moves.iter().eq(correct_moves.iter()));

        // ==== BLACK ====
        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 64);
        chess.set_piece_at(KING | BLACK, 4);
        chess.set_piece_at(PAWN | BLACK, 19);
        chess.set_piece_at(BISHOP | BLACK, 84);

        let moves = chess.generate_diagonal_sliding_moves(84);
        let correct_moves = [101, 118, 99, 114, 67, 50, 33, 16, 69, 54, 39];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
        chess.set_turn(BLACK);

        chess.set_piece_at(BISHOP, 53);
        chess.set_piece_at(KING | BLACK, 87);
        chess.set_piece_at(PAWN | BLACK, 70);
        chess.set_piece_at(BISHOP | BLACK, 98);

        let moves = chess.generate_diagonal_sliding_moves(98);
        let correct_moves = [115, 113, 81, 64, 83, 68, 53];

        assert!(moves.iter().eq(correct_moves.iter()));
    }
}

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x} - ", byte))?;
        }
        Ok(())
    }
}

fn main() {
    let mut chess = Chess::new();

    // chess.set_turn(BLACK);

    chess.set_piece_at(BISHOP, 53);
    chess.set_piece_at(KING, 51);
    chess.set_piece_at(PAWN, 68);
    // chess.set_piece_at(BISHOP | BLACK, 98);

    let moves = chess.generate_diagonal_sliding_moves(53);

    // chess.set_piece_at(BISHOP, 102);
    // chess.set_piece_at(KING, 112);
    // chess.set_piece_at(PAWN, 97);
    // chess.set_piece_at(BISHOP | BLACK, 82);

    // let moves = chess.generate_diagonal_sliding_moves(102 as u8);

    println!("{:?}", moves);
}
