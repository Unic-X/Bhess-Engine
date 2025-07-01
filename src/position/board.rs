use crate::{get_printable, piece::Piece};
use crate::{set_bit, Squares};

pub type Bitboard = u64;

#[derive(Debug)]
pub struct Board(pub [Bitboard; 12]);

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Sides {
    White,
    Black,
}

pub fn render(bitboard: Bitboard) {
    for rank in 0..8 {
        for file in 0..8 {
            //Use ranks and file to convert into Square number
            let square = rank * 8 + file;

            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            let printable_char = get_printable!(square, bitboard);
            print!(" {}", printable_char);
        }
        print!("\n");
    }
    print!("\n    a b c d e f g h \n");

    //Board state in u64 Decimal
    print!("Biboard : {bitboard} \n");
}

impl Board {
    pub fn empty() -> Self {
        Board([0; 12])
    }
    pub fn put_piece(&mut self, piece: Piece, square: Option<Squares>) {
        match square {
            Some(square) => {
                set_bit!(square, self.0[piece.index()]);
            }
            None => {
                eprintln!("put_piece: Imagine being so retard: Square is out of bounds")
            }
        };
    }
}
