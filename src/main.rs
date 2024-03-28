use piece::mask_pawn;

use crate::{utils::*,board::*};
mod board;
mod piece;
mod utils;
mod fen;
pub mod files;

#[allow(unused_variables)]

fn main() {

    let (attacks,masks) = init_slider_attacks(Slider::Bishop);
    let occupancy = set_bit!(&[Squares::d4]);
    render(get_bishop_attacks(Squares::d8, &occupancy, &masks, &attacks));
    render(mask_pawn(Squares::b4, piece::Sides::White));
    render(get_bishop_attacks(Squares::c5, &occupancy, &masks, &attacks));
}


