pub mod files;
use crate::files::{utils::*,board::*,piece::*};

#[allow(unused_variables)]
fn main() {
    let mut bitboards = [0;12];
    let mut occupancies:Vec<u64>; 

    let side:Option<Sides> = None;
    
    let enpassant:Option<Squares> = None;

    let castle:u8;
    set_bit!(&[Squares::e2],bitboards[0]);
    // Castle rights :
    // White King Side 1
    // White Queen Side 2
    // Black King Side 4
    // Black Queen Side 8
    //
    // See Castle enum 

    

    let (attacks,masks) = init_slider_attacks(Slider::Bishop);
    let occupancy = set_bit!(&[Squares::d4]);
    render_pieces(&bitboards);
    render(mask_pawn(Squares::b4, Sides::White));
    render(get_bishop_attacks(Squares::c5, &occupancy, &masks, &attacks));
}


