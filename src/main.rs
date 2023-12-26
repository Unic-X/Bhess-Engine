

mod board;

use std::time::SystemTime;

use board::{render, Squares};
use piece::*;

use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    let mut bitboard = 0;


    bitboard = set_bit!(Squares::d5);
    bitboard = set_bit!(Squares::d4);

 
    render(bitboard);
 
    let now = SystemTime::now();
    u64::count_ones(bitboard);
  println!("Took {:?}",now.elapsed());
    render(mask_knight(Squares::d4));
}


