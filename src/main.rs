use crate::{utils::*,board::*};
mod board;
mod piece;
mod utils;
mod fen;


#[allow(unused_variables)]

fn main() {
    init_e
    let (attacks,masks) = init_slider_attacks(Slider::Bishop);
    let occupancy = set_bit!(&[Squares::e7,Squares::d4]);
    get_bishop_attacks(Squares::d8, &occupancy, &masks, &attacks);

    get_bishop_attacks(Squares::c5, &occupancy, &masks, &attacks);
}


