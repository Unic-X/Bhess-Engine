use std::fmt::Display;
use crate::get_printable;
use crate::position::Position;

pub type Bitboard = u64;

#[derive(Debug)]
pub struct Board {
    pub position : Position,
    pub stack : Vec<Position>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Sides {
    White,
    Black,
}

impl Board {
    pub fn empty_bitboards() -> [Bitboard; 12] {
        [0; 12]
    }
    pub fn init() -> Board {
        Board {
            position : Position::new(),
            stack : Vec::new(),
        }
    }
    
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..12 {
            write!(f, "{:064b}", self.position.boards[i])?;
        }
        Ok(())
    }
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
