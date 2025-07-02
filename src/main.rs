mod defs;
mod piece;
mod position;
mod utils;

use position::*;
use utils::*;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unused_variables)]
fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let position = Position::from_str(defs::START_POS_FEN);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("end - start = {:?}", end - start);
    let bitboard: u64 = 0;
    match position {
        Ok(pos) => {//time this
            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            pos.render_fancy();
            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            println!("end - start = {:?}", end - start);
        }
        Err(_) => {}
    }
}
