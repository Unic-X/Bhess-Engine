mod board;

use std::time::SystemTime;

use board::{render, Squares};
use piece::*;

use crate::utils::*;
mod piece;
mod utils;

#[allow(unused_variables)]

fn main() {


    for board in init_slider_attacks(Slider::Bishop).iter(){
        render(*board);
    };   
}


