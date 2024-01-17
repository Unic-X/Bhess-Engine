mod board;
use board::{render, Squares};

use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    let (attacks,masks) = init_slider_attacks(Slider::Bishop);
    use std::time::Instant;
    let occupancy = set_bit!(&[Squares::e7,Squares::d4]);
    get_bishop_attacks(Squares::d8, &occupancy, &masks, &attacks);
    for _ in 1..1000{

    let now = Instant::now();
    get_bishop_attacks(Squares::c5, &occupancy, &masks, &attacks);
    println!("{:?}",now.elapsed());
    }
}


