use crate::get_printable;

pub type Bitboard = u64;

pub struct Board{
    pieces: [Bitboard; 12],
}

#[derive(PartialEq, Eq,Hash)]
pub enum Sides {
    White,
    Black,
}

impl Board {
    pub fn new()->Self{
        Board { 
             pieces : [0;12]
        }
    }   
    pub fn render_all(&self){
        for boards in self.pieces {
            render(boards);
        }   
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


