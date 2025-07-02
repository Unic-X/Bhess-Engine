use std::fmt::Display;
use crate::position::Position;

pub type Bitboard = u64;

#[derive(Debug)]
pub struct Board {
    pub position : Position,
    pub stack : Vec<Position>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Sides {
    White,
    Black,
}

impl Board {
    pub fn empty_bitboards() -> [Bitboard; 12] {
        [0; 12]
    }
    pub fn init() -> Board {
        Board {
            position : Position::new(),
            stack : Vec::new(),
        }
    }
    
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..12 {
            write!(f, "{:064b}", self.position.boards[i])?;
        }
        Ok(())
    }
}