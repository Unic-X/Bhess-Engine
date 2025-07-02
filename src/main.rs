mod defs;
mod piece;
mod position;
mod utils;

use position::*;
use utils::*;
use std::str::FromStr;

#[allow(unused_variables)]

fn main() {
    let position = Position::from_str(defs::COMPLEX_POS_FEN);
    match position {
        Ok(pos) => {//time this
            pos.render_fancy();
        }
        Err(_) => {}
    }
    
}
