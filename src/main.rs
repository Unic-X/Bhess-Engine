mod defs;
mod piece;
mod position;
mod utils;

use position::*;
use utils::*;

use crate::{
    piece::attacks::get_bishop_attacks,
    utils::magic::init_slider_attacks,
};

#[allow(unused_variables)]

fn main() {
    let bitboard = set_bit!(vec![Squares::d6, Squares::g8, Squares::f3, Squares::e5]);
    let (b_attacks, b_masks) = init_slider_attacks(Slider::Bishop);

    let (r_attacks, r_masks) = init_slider_attacks(Slider::Rook);

    get_bishop_attacks(
        Squares::d5,
        &bitboard,
        &b_masks,
        &b_attacks,
    );

    
}
