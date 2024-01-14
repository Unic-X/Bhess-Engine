mod board;
use board::{render, Squares};

use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    let (attacks,masks) = init_slider_attacks(Slider::Bishop);
    let occupancy = set_bit!(&[Squares::b5,Squares::f5,Squares::c3,Squares::f2,Squares::g7]);
    render(occupancy);
    use std::time::Instant;
    let now = Instant::now();
    // render(ratt(Squares::e5 as u8, occupancy));
    render(get_bishop_attacks(Squares::f4, occupancy, masks, attacks));
    println!("{:?}",now.elapsed());

}


