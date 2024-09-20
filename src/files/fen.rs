use crate::{set_bit, files::{board::BoardState, piece::Sides}};

use super::{board::{Bitboard, Squares}, piece::{Piece, Castle}};

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_KIWIPETE_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";




///
/// Private function used to parse castle_rights using the FEN string
/// 
/// Example
/// ```
/// Will add later
/// 

fn parse_castle_rights(fen_str:&str)->Castle{
    unimplemented!()
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

fn parse_fen(fen_str:&mut str)->BoardState{
    let mut board_state = BoardState::new();
       
    let rank_state :Vec<&str> = fen_str.split(r" ").collect();
    let rank = rank_state.iter().nth(0);

    match rank {
        Some(r) => {
            let ranks_fen:Vec<&str>;
            ranks_fen = r.split(r"/").collect();
            let mut current_rank = 0;
            for rank in ranks_fen{
                current_rank += 1;
                for piece in Piece::gen_all(){
                    parse_piece_rank(rank, current_rank, piece.simple_char());
                }
            }

        },
        None => {
            eprintln!("Error reading FEN string issue while reading RANK");
            panic!("Failed to read the FEN string fmt issue");  // Currently using Panic later will
            // use option as return to check if its ok or not
        },
    };
        

    if let Some(&next) = rank_state.iter().nth(1){
        match next {
            "W" | "w" => {
                board_state.turn = Sides::White;
            },
            "B" | "b" => {
                board_state.turn = Sides::Black;
            },
            _ => {}
        }
    }
        
    unimplemented!()
}


