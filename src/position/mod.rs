mod castling;
mod board;
mod fen;
mod square;

pub use board::*;
pub use castling::Castle;
pub use square::Squares;

pub struct Position {
    pub board: Board,
    pub colour_to_move: Sides,
    pub castling_rights: Castle,
    pub en_passant_square: Option<Squares>,
    pub half_move_clock: u8,
    pub full_move_counter: u8,
}
