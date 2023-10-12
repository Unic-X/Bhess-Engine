use crate::board::Squares;
use crate::board::render;
use crate::set_bit;

pub enum Sides{
    White,
    Black,
}


pub fn main() {
    mask_pawn(&[Squares::a3,Squares::b7], Sides::White,128);

}

pub fn mask_pawn(sq:&[Squares],side:Sides,mut bitboard:u64)->u64{
    let mut attacks:u64 = 0;

    set_bit!(sq,mut bitboard);

    match side {
        Sides::White=>{
            attacks |= (bitboard<<7);
            return attacks;
        },
        Sides::Black=>{
    
            attacks |= (bitboard<<7);
            return attacks;
            
        }
    
    }

}
