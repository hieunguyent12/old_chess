const EMPTY: u8 = 0; //         00000000
const PAWN: i8 = 1; //          00000001
const ROOK: i8 = 2; //          00000010
const KNIGHT: i8 = 4; //        00000100
const BISHOP: u8 = 8; //        00001000
const KING: u8 = 16; //         00010000
const QUEEN: i8 = 32; //        00100000

// const PIECE_MASK: i8 = 63; //   00111111
const COLOR_MASK: u8 = 128; // 10000000

const BOARD_SIZE: u8 = 128;

const BISHOP_DELTAS: [i8; 4] = [17, 15, -17, -15];

/*
https://github.com/jhlywa/chess.js/blob/master/src/chess.ts#L169C1-L178C2
{
  a8:   0, b8:   1, c8:   2, d8:   3, e8:   4, f8:   5, g8:   6, h8:   7,
  a7:  16, b7:  17, c7:  18, d7:  19, e7:  20, f7:  21, g7:  22, h7:  23,
  a6:  32, b6:  33, c6:  34, d6:  35, e6:  36, f6:  37, g6:  38, h6:  39,
  a5:  48, b5:  49, c5:  50, d5:  51, e5:  52, f5:  53, g5:  54, h5:  55,
  a4:  64, b4:  65, c4:  66, d4:  67, e4:  68, f4:  69, g4:  70, h4:  71,
  a3:  80, b3:  81, c3:  82, d3:  83, e3:  84, f3:  85, g3:  86, h3:  87,
  a2:  96, b2:  97, c2:  98, d2:  99, e2: 100, f2: 101, g2: 102, h2: 103,
  a1: 112, b1: 113, c1: 114, d1: 115, e1: 116, f1: 117, g1: 118, h1: 119
}
 */

// why does it have 239 items?
// because of how the indexes of the squares on the real board are laid out due to the fact that
// we have to use some indexes in between to represent the dummy board
/*
  q & b = 40
  q & r = 34
  q & b & k = 56

  queen, pawn, bishop, king can move diagonally one square = 57
  00100000
  00000001
  00001000
  00010000

  00111001

  what is the signifance of the fact that the diff. between each square is unique?
  because they are unique, we can store every single diff. (offset by 119) in an array for lookup.
  This way, we can quickly check if a square can be attacked by just finding the difference between the indexes
*/
#[rustfmt::skip]
const ATTACKS: [u8; 239]= [
   1, 0, 0, 0, 0, 0, 0, 24,  0, 0, 0, 0, 0, 0,20, 0, // Notice how the non-zero numbers are placed very specifically,
   0, 1, 0, 0, 0, 0, 0, 24,  0, 0, 0, 0, 0,20, 0, 0, 
   0, 0, 1, 0, 0, 0, 0, 24,  0, 0, 0, 0,20, 0, 0, 0,
   0, 0, 0, 1, 0, 0, 0, 24,  0, 0, 0,20, 0, 0, 0, 0,
   0, 0, 0, 0, 1, 0, 0, 24,  0, 0,20, 0, 0, 0, 0, 0,
   0, 0, 0, 0, 0, 1, 2, 24,  2,20, 0, 0, 0, 0, 0, 0,
   0, 0, 0, 0, 0, 2,53, 56, 53, 2, 0, 0, 0, 0, 0, 0,
  24,24,24,24,24,24,56,  0, 56,24,24,24,24,24,24, 0, // Note the zero in the very middle, it basically represents the current piece that is being evaluated for attacks
   0, 0, 0, 0, 0, 2,53, 56, 53, 2, 0, 0, 0, 0, 0, 0, // But the piece isn't always in the middle? We can "move" it to the middle by adding 119
   0, 0, 0, 0, 0,20, 2, 24,  2,20, 0, 0, 0, 0, 0, 0, // and then applying the difference between two squares to find the index relative to the piece in the middle
   0, 0, 0, 0,20, 0, 0, 24,  0, 0,20, 0, 0, 0, 0, 0,
   0, 0, 0,20, 0, 0, 0, 24,  0, 0, 0,20, 0, 0, 0, 0,
   0, 0,20, 0, 0, 0, 0, 24,  0, 0, 0, 0,20, 0, 0, 0,
   0,20, 0, 0, 0, 0, 0, 24,  0, 0, 0, 0, 0,20, 0, 0,
  20, 0, 0, 0, 0, 0, 0, 24,  0, 0, 0, 0, 0, 0,20, 
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

struct Chess {
    board: [u8; BOARD_SIZE as usize],

    /// 0 = white, 1 = black
    turn: u8,

    history: Vec<History>,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            board: [0; BOARD_SIZE as usize],
            turn: 0,
            history: vec![],
        }
    }

    pub fn move_piece(&mut self, from_idx: u8, to_idx: u8) {
        // TODO: move the piece as long as the king is not in checked
        if self.is_on_board(to_idx)
        /* && self.get_square_at(to_idx) == 0 */
        {
            let piece = self.get_square_at(from_idx);
            let to_piece = self.get_square_at(to_idx);

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

    pub fn get_square_at(&self, square_idx: u8) -> u8 {
        self.board[square_idx as usize]
    }

    pub fn set_piece_at(&mut self, piece: u8, square_idx: u8) {
        self.board[square_idx as usize] = piece;
    }

    pub fn generate_diagonal_sliding_moves(&mut self, square_idx: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = vec![];

        for delta in BISHOP_DELTAS {
            let mut isBlocked = false;
            let mut destination_idx = square_idx as i16 + delta as i16;

            while self.is_on_board(destination_idx as u8) {
                let square = self.get_square_at(destination_idx as u8);

                // if we encounter a friendly piece, we can't move there
                if square != EMPTY && (square & COLOR_MASK) == 0 {
                    break;
                }

                // if we encounter an enemy piece, we are blocked and cannot move further
                if square != EMPTY && (square) == BISHOP | COLOR_MASK {
                    moves.push(destination_idx as u8);
                    break;
                }

                // TODO: check if this move would leave the king in check?

                // play the move
                self.move_piece(square_idx, destination_idx as u8);
                // check if the king is in check

                // TODO: store the king's position somehow, for now, we just hardcode in 4

                let mut skip = false;

                // TODO: loop through all the enemy pieces? - maybe do this AFTER we generate all the moves so we only loop once?
                for idx in 0..BOARD_SIZE {
                    // if piece is friendly, skip
                    if self.get_square_at(idx) == BISHOP {
                        continue;
                    }

                    let piece = self.get_square_at(idx);

                    // if piece is enemy, check if it can attack the king using the king's index
                    if piece == BISHOP | COLOR_MASK && Self::is_attacked(0x00 as u8, idx) {
                        println!("King is attacked!");
                        println!(
                            "Enemy: {:x} - Destination: {:x} - From: {:x}",
                            idx, destination_idx, square_idx
                        );
                        skip = true;

                        break;
                    }
                }

                // revert the move
                self.undo();

                if !skip {
                    moves.push(destination_idx as u8);

                    // if the destination square is on the board, we keep searching in that direction until we go off the board
                    destination_idx += delta as i16;
                } else {
                    break;
                }
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

    pub fn is_attacked(defender_idx: u8, attacker_idx: u8) -> bool {
        let diff = defender_idx as i8 - attacker_idx as i8 + 119;

        ATTACKS[diff as usize] != 0
    }

    pub fn current_turn(&self) {
        match self.turn {
            0 => {}
            1 => {}
            _ => panic!("invalid turn"),
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
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

        chess.set_piece_at(BISHOP, 0x34 as u8);
        chess.set_piece_at(KING, 0x25 as u8);
        chess.set_piece_at(BISHOP | COLOR_MASK, 0x56 as u8);

        let moves = chess.generate_diagonal_sliding_moves(0x34 as u8);

        let correct_moves = [69, 86, 67, 82, 97, 112, 35, 18, 1];

        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();

        let correct_moves = [17, 34, 51, 68, 85, 102, 119];

        chess.set_piece_at(BISHOP, 0x00 as u8);
        chess.set_piece_at(KING, 0x01 as u8);
        chess.set_piece_at(BISHOP | COLOR_MASK, 0x77 as u8);

        let moves = chess.generate_diagonal_sliding_moves(0x00 as u8);
        assert!(moves.iter().eq(correct_moves.iter()));

        chess.clear();
    }

    #[test]
    fn bishop_can_not_move_because_king_is_checked() {
        let mut chess = Chess::new();

        chess.set_piece_at(BISHOP, 0x70 as u8);
        chess.set_piece_at(KING, 0x43 as u8);
        // 52
        chess.set_piece_at(BISHOP | COLOR_MASK, 0x54 as u8);

        let moves = chess.generate_diagonal_sliding_moves(0x34 as u8);

        let correct_moves: [u8; 0] = [];

        assert!((moves.len() == 0) && (correct_moves.len() == 0));
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

    chess.set_piece_at(BISHOP, 0x15 as u8);
    chess.set_piece_at(KING, 0x00 as u8);
    chess.set_piece_at(BISHOP | COLOR_MASK, 0x77 as u8);

    let moves = chess.generate_diagonal_sliding_moves(0x15 as u8);

    // chess.set_piece_at(BISHOP, 0x34 as u8);
    // chess.set_piece_at(KING, 0x25 as u8);
    // chess.set_piece_at(BISHOP | COLOR_MASK, 0x56 as u8);

    // let moves = chess.generate_diagonal_sliding_moves(0x34 as u8);
    println!("{:?}", moves);
    let hexs = ByteBuf(moves.as_slice());

    println!("{:x}", hexs);

    // println!("{:?}", chess.board);

    // 36, 2

    // let rank = 15;
    // let file = 7;
    // // the width of the board is 16 since there is a dummy board adjacent to the real board
    // let index = rank * 16 + file;
    // // we use hex 80 to accomodate 128 indexes in the array
    // let b = 0x80; // 0b10000000; 0b1000
    //               // 10001000
    //               // 10000100

    // /*

    // The 0x88 system solves this problem.  By using a 16 x 8 board, you get a marker bit.
    // You can tell if you've gone off into the unused right board, because the 0x08 bit is set if you've done this.
    // h1 is 7, if you add one you get 8, which has the 0x08 bit set.
    // None of the "left" (real) board squares has the 0x08 bit set, and all of the "right" (dummy) board squares have this bit set.
    // If you are on a3 and you try to go to the left one square, you're on p2, which is in the dummy board, which has the 0x08 bit set.

    // */

    // // this only works up to index 255 since 0x88 only evaluate to an 8-bit binary number
    // let isOffTheBoard = (index & 0x88);

    // println!("index: {} / {}", index, isOffTheBoard);
}
