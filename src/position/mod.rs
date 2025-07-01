mod board;
mod castling;
mod fen;
mod square;

pub use board::*;
pub use castling::CastleRights;
pub use square::Squares;

#[derive(Debug)]
pub struct Position {
    pub board: Board,
    pub colour_to_move: Sides,
    pub castling_rights: CastleRights,
    pub en_passant_square: Option<Squares>,
    pub half_move_clock: u8,
    pub full_move_counter: u8,
}
