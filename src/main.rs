

mod board;
use board::{render, Squares};
use piece::*;

use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    render(mask_pawn(Squares::d4,Sides::White));
}


