use std::{ops::Shl, str::FromStr};
use strum_macros::EnumIter;

use crate::Bitboard;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone,Debug,EnumIter,PartialEq)]
pub enum Squares {
    a8,b8,c8,d8,e8,f8,g8,h8,
    a7,b7,c7,d7,e7,f7,g7,h7,
    a6,b6,c6,d6,e6,f6,g6,h6,    
    a5,b5,c5,d5,e5,f5,g5,h5,
    a4,b4,c4,d4,e4,f4,g4,h4,
    a3,b3,c3,d3,e3,f3,g3,h3,
    a2,b2,c2,d2,e2,f2,g2,h2,
    a1,b1,c1,d1,e1,f1,g1,h1,
}

impl FromStr for Squares{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a8" => Ok(Squares::a8), "b8" => Ok(Squares::b8), "c8" => Ok(Squares::c8), "d8" => Ok(Squares::d8),
            "e8" => Ok(Squares::e8), "f8" => Ok(Squares::f8), "g8" => Ok(Squares::g8), "h8" => Ok(Squares::h8),

            "a7" => Ok(Squares::a7), "b7" => Ok(Squares::b7), "c7" => Ok(Squares::c7), "d7" => Ok(Squares::d7),
            "e7" => Ok(Squares::e7), "f7" => Ok(Squares::f7), "g7" => Ok(Squares::g7), "h7" => Ok(Squares::h7),

            "a6" => Ok(Squares::a6), "b6" => Ok(Squares::b6), "c6" => Ok(Squares::c6), "d6" => Ok(Squares::d6),
            "e6" => Ok(Squares::e6), "f6" => Ok(Squares::f6), "g6" => Ok(Squares::g6), "h6" => Ok(Squares::h6),

            "a5" => Ok(Squares::a5), "b5" => Ok(Squares::b5), "c5" => Ok(Squares::c5), "d5" => Ok(Squares::d5),
            "e5" => Ok(Squares::e5), "f5" => Ok(Squares::f5), "g5" => Ok(Squares::g5), "h5" => Ok(Squares::h5),

            "a4" => Ok(Squares::a4), "b4" => Ok(Squares::b4), "c4" => Ok(Squares::c4), "d4" => Ok(Squares::d4),
            "e4" => Ok(Squares::e4), "f4" => Ok(Squares::f4), "g4" => Ok(Squares::g4), "h4" => Ok(Squares::h4),

            "a3" => Ok(Squares::a3), "b3" => Ok(Squares::b3), "c3" => Ok(Squares::c3), "d3" => Ok(Squares::d3),
            "e3" => Ok(Squares::e3), "f3" => Ok(Squares::f3), "g3" => Ok(Squares::g3), "h3" => Ok(Squares::h3),

            "a2" => Ok(Squares::a2), "b2" => Ok(Squares::b2), "c2" => Ok(Squares::c2), "d2" => Ok(Squares::d2),
            "e2" => Ok(Squares::e2), "f2" => Ok(Squares::f2), "g2" => Ok(Squares::g2), "h2" => Ok(Squares::h2),

            "a1" => Ok(Squares::a1), "b1" => Ok(Squares::b1), "c1" => Ok(Squares::c1), "d1" => Ok(Squares::d1),
            "e1" => Ok(Squares::e1), "f1" => Ok(Squares::f1), "g1" => Ok(Squares::g1), "h1" => Ok(Squares::h1),

            _ => Err(format!("Invalid square: {}", s)),
        }
    }
}

impl Squares {
    // Converts u8 to Squares enum by manual matching
    pub fn from_u8(sq: u8) -> Option<Self> {
        match sq {
            0 => Some(Squares::a8),
            1 => Some(Squares::b8),
            2 => Some(Squares::c8),
            3 => Some(Squares::d8),
            4 => Some(Squares::e8),
            5 => Some(Squares::f8),
            6 => Some(Squares::g8),
            7 => Some(Squares::h8),

            8 => Some(Squares::a7),
            9 => Some(Squares::b7),
            10 => Some(Squares::c7),
            11 => Some(Squares::d7),
            12 => Some(Squares::e7),
            13 => Some(Squares::f7),
            14 => Some(Squares::g7),
            15 => Some(Squares::h7),

            16 => Some(Squares::a6),
            17 => Some(Squares::b6),
            18 => Some(Squares::c6),
            19 => Some(Squares::d6),
            20 => Some(Squares::e6),
            21 => Some(Squares::f6),
            22 => Some(Squares::g6),
            23 => Some(Squares::h6),

            24 => Some(Squares::a5),
            25 => Some(Squares::b5),
            26 => Some(Squares::c5),
            27 => Some(Squares::d5),
            28 => Some(Squares::e5),
            29 => Some(Squares::f5),
            30 => Some(Squares::g5),
            31 => Some(Squares::h5),

            32 => Some(Squares::a4),
            33 => Some(Squares::b4),
            34 => Some(Squares::c4),
            35 => Some(Squares::d4),
            36 => Some(Squares::e4),
            37 => Some(Squares::f4),
            38 => Some(Squares::g4),
            39 => Some(Squares::h4),

            40 => Some(Squares::a3),
            41 => Some(Squares::b3),
            42 => Some(Squares::c3),
            43 => Some(Squares::d3),
            44 => Some(Squares::e3),
            45 => Some(Squares::f3),
            46 => Some(Squares::g3),
            47 => Some(Squares::h3),

            48 => Some(Squares::a2),
            49 => Some(Squares::b2),
            50 => Some(Squares::c2),
            51 => Some(Squares::d2),
            52 => Some(Squares::e2),
            53 => Some(Squares::f2),
            54 => Some(Squares::g2),
            55 => Some(Squares::h2),

            56 => Some(Squares::a1),
            57 => Some(Squares::b1),
            58 => Some(Squares::c1),
            59 => Some(Squares::d1),
            60 => Some(Squares::e1),
            61 => Some(Squares::f1),
            62 => Some(Squares::g1),
            63 => Some(Squares::h1),

            _ => None, // Out of bounds
        }
    }
    pub fn rank(&self)->u8{
        (*self as u8) / 8 + 1
    }

    // Generic function to convert any unsigned integer to Squares
    pub fn from<T>(sq: T) -> Option<Self>
    where
        T: Into<u64>,     
    {
        let value = sq.into();
        if value < 64 {
            Squares::from_u8(value as u8)
        } else {
            None
        }
    }
}

impl Shl<Squares> for Bitboard {
        type Output = Bitboard;

        fn shl(self, square: Squares) -> Self::Output {
            self << square as u8
        }
}