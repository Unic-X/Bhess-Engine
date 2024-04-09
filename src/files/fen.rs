use nom::IResult;

use super::{board::{Squares, Bitboard}, piece::{Castle, Sides}};

/// Defines the Result after successful FEN parsed string

pub struct FenRes{
    enpassant : Option<Squares>,     // Enpassant Square
    castle_squares : Option<Castle>, // Castle Squares 
    side : Sides,              // Active side
    placement : Vec<Bitboard>, // All 12 Bitboards 
}

fn parse_fen(input:&str)->IResult<&str,FenRes>{
    //Fen is 
}

