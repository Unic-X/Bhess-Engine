use crate::{get_printable, piece::Piece};
use crate::Squares;

pub type Bitboard = u64;

pub struct Board([Bitboard;12]);

#[derive(PartialEq, Eq,Hash)]
pub enum Sides {
    White,
    Black,
}

pub fn render(bitboard:Bitboard){
 for rank in 0..8 {
        for file in 0..8 {
            //Use ranks and file to convert into Square number
            let square = rank * 8 + file;
            
            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            print!(" {}",get_printable!(square, bitboard));

        }
        print!("\n");
    }
    print!("\n    a b c d e f g h \n");
    
    //Board state in u64 Decimal 
    print!("Biboard : {bitboard} \n");
}

impl Board {
    pub fn empty()->Self{
        Board([0;12])
    }
    pub fn put_piece(&mut self, piece: Piece, square: Option<Squares>) {
        match square {
            Some(square) => {self.0[piece.index()] |= square as u64} 
            None => {eprintln!("put_piece: Imagine being so retard: Square is out of bounds")}
        };
    }
}
