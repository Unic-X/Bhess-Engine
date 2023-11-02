

mod board;
use crate::board::*;
use crate::piece::*;
use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    
    let bitboard = 4624614895390720;
    render(bitboard);
    
    println!("{:?} {:?} {:?} ",bit_count(bitboard),Squares::e7 as u64,get_lsb(bitboard).unwrap());
}


