mod position;
mod utils;
mod piece;
mod defs;
  
use std::time::{Instant, SystemTime};

use position::*;
use piece::*;
use utils::{magic::*,*};

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
    let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let start = Instant::now();
    // let rook_bitboard = get_rook_attacks(Squares::f6,&bitboard,&r_masks, &r_attacks);
    let pawn_bitboard = attacks::mask_pawn(Squares::e2, Sides::White);
    let elapsed = start.elapsed();
    render(pawn_bitboard);
    println!("mask_queen execution time: {:?}", elapsed);
}


