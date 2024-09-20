pub mod files;
use crate::files::{utils::*,board::*,piece::*};

#[allow(unused_variables)]
fn main() {
     // Castle rights :
    // White King Side 1
    // White Queen Side 2
    // Black King Side 4
    // Black Queen Side 8
    //
    // See Castle enum 

    let bitboard = set_bit!(&[Squares::d6,Squares::g8,Squares::f3,Squares::e5]);
    render(bitboard);

    let (b_attacks,b_masks) = init_slider_attacks(Slider::Bishop);
    let (r_attacks,r_masks) = init_slider_attacks(Slider::Rook);

    render(mask_queen(Squares::d5,&bitboard, &b_masks, &b_attacks, &r_masks, &r_attacks));
    
    let occupancy = set_bit!(&[Squares::d4]);
}


