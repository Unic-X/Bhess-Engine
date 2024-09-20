use std::{ops::Shl, collections::HashMap};
use crate::{get_bit, files::piece::Piece, get_printable};
use strum_macros::EnumIter;

use super::piece::{Sides, Castle};



pub type Bitboard = u64;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone,Debug,EnumIter)]
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


pub struct BoardState{
    pub bitboards : HashMap<Piece,Bitboard>,
    pub turn: Sides,
    pub castle : Castle,
    pub half_move : u8,


}

impl BoardState {
    pub fn new()->Self{
        BoardState { 
            bitboards: {
                let mut _piece_hashmap = HashMap::new();
                for piece in Piece::gen_all(){
                    _piece_hashmap.insert(piece,0);
                }
                _piece_hashmap
            }, 
            turn: Sides::White, 
            castle: Castle::WK | Castle::WQ | Castle::BK | Castle::BQ, 
            half_move: 0 
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

    // Generic function to convert any unsigned integer to Squares
    pub fn from<T>(sq: T) -> Option<Self>
    where
        T: Into<u64>, // Convert into u64 to handle large types
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

pub fn render(bitboard:Bitboard){
 for rank in 0..8 {
        for file in 0..8 {
            //Use ranks and file to convert into Square number
            let square = rank * 8 + file;
            
            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            print!(" {}",get_printable!(square, bitboard));

        }
        print!("\n");
    }
    print!("\n    a b c d e f g h \n");
    
    //Board state in u64 Decimal 
    print!("Biboard : {bitboard} \n");
}



pub fn render_pieces(bitboards:&[Bitboard]){
    let all_pieces = Piece::gen_all();
    for rank in 0..8 {
        for file in 0..8 {
            //Use ranks and file to convert into Square number
            let square = rank * 8 + file;
            
            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            
            let mut piece = None;

            for bb_piece in &all_pieces{
                if get_bit!(square, bitboards[bb_piece.index()]) {
                    piece = Some(bb_piece);
                }
            }

            match piece {
                Some(p) => {
                    print!(" {}",p.simple_char());
                }
                None => {
                    print!(" .");
                }
            }
           
        }
        print!("\n");
    }
    print!("\n    a b c d e f g h \n");
    
}

