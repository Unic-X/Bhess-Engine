

mod board;
use board::display;
use board::Squares;
use utils::bit_count;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {

    let bitboard = 4624614895390720;
    display(&[Squares::h8]);
    println!("{:?}",bit_count(bitboard));
}


