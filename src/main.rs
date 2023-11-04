

mod board;
use crate::board::*;
use crate::piece::*;
use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {
    let state = 1804289383; 
    render(get_random_64(state) as u64);
}


