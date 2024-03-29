pub mod files;
use crate::files::{utils::*,board::*,piece::*};

#[allow(unused_variables)]
fn main() {
    let mut bitboards = [0;12];
    let mut occupancies:Vec<u64>; 

    let side:Option<Sides> = None;
    
    let enpassant:Option<Squares> = None;

    let castle:u8;

    set_bit!(&[Squares::a2],bitboards[0]);
    set_bit!(&[Squares::b2],bitboards[0]);
    set_bit!(&[Squares::c2],bitboards[0]);
    set_bit!(&[Squares::d2],bitboards[0]);
    set_bit!(&[Squares::e2],bitboards[0]);
    set_bit!(&[Squares::f2],bitboards[0]);
    set_bit!(&[Squares::g2],bitboards[0]);
    set_bit!(&[Squares::h2],bitboards[0]);


    set_bit!(&[Squares::a1],bitboards[3]);
    set_bit!(&[Squares::h1],bitboards[3]);

    set_bit!(&[Squares::g1],bitboards[1]);
    set_bit!(&[Squares::b1],bitboards[1]);

    set_bit!(&[Squares::f1],bitboards[2]);
    set_bit!(&[Squares::c1],bitboards[2]);

    set_bit!(&[Squares::e1],bitboards[5]);
    
    set_bit!(&[Squares::d1],bitboards[4]);




    set_bit!(&[Squares::a7],bitboards[6]);
    set_bit!(&[Squares::b7],bitboards[6]);
    set_bit!(&[Squares::c7],bitboards[6]);
    set_bit!(&[Squares::d7],bitboards[6]);
    set_bit!(&[Squares::e7],bitboards[6]);
    set_bit!(&[Squares::f7],bitboards[6]);
    set_bit!(&[Squares::g7],bitboards[6]);
    set_bit!(&[Squares::h7],bitboards[6]);


    set_bit!(&[Squares::a8],bitboards[9]);
    set_bit!(&[Squares::h8],bitboards[9]);

    set_bit!(&[Squares::g8],bitboards[7]);
    set_bit!(&[Squares::b8],bitboards[7]);

    set_bit!(&[Squares::f8],bitboards[8]);
    set_bit!(&[Squares::c8],bitboards[8]);

    set_bit!(&[Squares::e8],bitboards[11]);
    
    set_bit!(&[Squares::d8],bitboards[10]);


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
}


