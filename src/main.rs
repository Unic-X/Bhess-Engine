mod board;
use board::{render, Squares};
use samaj::piece::NOT_A_FILE;

use crate::{utils::*, piece::mask_king};
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    let (attacks,masks) = init_slider_attacks(Slider::Bishop);
    use std::time::Instant;
    let now = Instant::now();
    let a = mask_king(Squares::a8);
    render(NOT_A_FILE);
    render(a);
}


