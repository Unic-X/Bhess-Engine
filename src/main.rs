mod defs;
mod piece;
mod position;
mod utils;

use position::*;
use utils::*;
use std::str::FromStr;

use crate::piece::attacks::mask_knight;
use crate::piece::Piece;

#[allow(unused_variables)]
fn main() {

    let position = Position::from_str(defs::COMPLEX_POS_FEN);
    let bitboard: u64 = 0;
    match position {
        Ok(pos) => {
            let bitboard = pos.board;
            println!("{:?}", pos.colour_to_move);
            render(bitboard.0[1]);
        }
        Err(_) => {}
    }



}
