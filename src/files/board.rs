use std::ops::Shl;
use crate::get_bit;
use strum_macros::EnumIter;


#[allow(non_camel_case_types)]
#[derive(Copy, Clone,Debug,EnumIter)]
pub enum Squares {
    a8,b8,c8,d8,e8,f8,g8,h8,
    a7,b7,c7,d7,e7,f7,g7,h7,
    a6,b6,c6,d6,e6,f6,g6,h6,    
    a5,b5,c5,d5,e5,f5,g5,h5,
    a4,b4,c4,d4,e4,f4,g4,h4,
    a3,b3,c3,d3,e3,f3,g3,h3,
    a2,b2,c2,d2,e2,f2,g2,h2,
    a1,b1,c1,d1,e1,f1,g1,h1,
}


impl Shl<Squares> for u64 {
        type Output = u64;

        fn shl(self, square: Squares) -> Self::Output {
            self << square as u8
        }
}

pub fn render(bitboard:u64){
 for rank in 0..8 {
        for file in 0..8 {
            //Use ranks and file to convert into Square number
            let square = rank * 8 + file;
            
            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            print!(" {}",get_bit!(square, bitboard));

            /*match bitboard & (1 << square) {
                0 => print!("0"),
                _ => print!("1"),
            }*/
        }
        print!("\n");
    }
    print!("\n    a b c d e f g h \n");
    
    //Board state in u64 Decimal 
    print!("Biboard : {bitboard} \n");
}

/// Sets the initial bitboard for all 12 pieces
///
/// White : King, Queen, Rook, Knight, Bishop, Pawn
/// Black : King, Queen, Rook, Knight, Bishop, Pawn
///
/// Example: 
/// 
/// 
///
fn init_board(){

}
