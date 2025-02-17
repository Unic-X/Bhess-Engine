use crate::piece::Piece;
use crate::position::Position;
use crate::position::Squares;
use crate::set_bit;

use crate::Bitboard;
use crate::Castle;

use super::Sides;

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_KIWIPETE_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

fn parse_castle_rights(fen_str: &str) -> Vec<Castle> {
    let mut rights = Vec::new();

    for ch in fen_str.trim().chars() {
        match ch {
            'K' => rights.push(Castle::WK),
            'Q' => rights.push(Castle::WQ),
            'k' => rights.push(Castle::BK),
            'q' => rights.push(Castle::BQ),
            _ => continue,
        }
    }

    if rights.is_empty() {
        rights.push(Castle::NA);
    }

    rights
}


///
/// This function is used to parse single rank string and returns the bitboard for a single piece type 
/// `OR` the bitboard returned for each iteration to get the final bitboard 
/// Example
/// ```
/// Will add later
/// ```

fn parse_piece_rank(fen_str:&str,rank:u8, p:char)->Bitboard{
    let mut bitboard = 0;
    for (i,c) in fen_str.chars().enumerate(){
        if p == c {
            set_bit!(Squares::from((rank*8)+i as u8),bitboard);
        }
    }
    bitboard
}

///
/// Parse the FEN string and return a `mutable` reference of BoardState should be added inside the
/// impl of board_state to `mutate` &self instead will change this later
/// 
/// Example
/// ```
/// unimplemented!()
/// ```

fn parse_fen(fen_str:&mut str)->Position{
    //
    todo!()  
}
