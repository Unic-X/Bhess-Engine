use crate::position::*;

#[derive(Debug)]
pub struct Position {
    pub boards: [Bitboard;12],
    pub colour_to_move: Sides,
    pub castling_rights: CastleRights,
    pub en_passant_square: Option<Squares>,
    pub half_move_clock: u8,
    pub full_move_counter: u8,
    pub material_score : [f64;2],
}
