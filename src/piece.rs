use crate::board::Squares;
use crate::set_bit;

pub enum Sides {
    White,
    Black,
}

pub const NOT_A_FILE: u64 = 18374403900871474942;

// not H file constant
pub const NOT_H_FILE: u64 = 9187201950435737471;

// not HG file constant
pub const NOT_HG_FILE: u64 = 4557430888798830399;

// not AB file constant
pub const NOT_AB_FILE: u64 = 18229723555195321596;

pub fn mask_pawn(sq: &[Squares], side: Sides, mut bitboard: u64) -> u64 {
    let mut attacks: u64 = 0;

    set_bit!(sq, mut bitboard);

    match side {
        Sides::White => {
            if ((bitboard >> 7) & NOT_A_FILE) > 0 {
                attacks |= bitboard >> 7;
            } else {
                return bitboard;
            }
            if ((bitboard >> 9) & NOT_H_FILE) > 0 {
                attacks |= bitboard >> 9;
            } else {
                return bitboard;
            }
        }
        Sides::Black => {
            if ((bitboard << 7) & NOT_H_FILE) > 0 {
                attacks |= bitboard << 7;
            } else {
                return bitboard;
            }
            if ((bitboard << 9) & NOT_A_FILE) > 0 {
                attacks |= bitboard << 9;
            } else {
                return bitboard;
            }
        }
    }
    return attacks;
}

pub fn mask_knight(sq: &[Squares], mut bitboard: u64) -> u64 {
    let mut attacks: u64 = 0;

    set_bit!(sq, mut bitboard);
    if ((bitboard >> 17) & NOT_H_FILE) > 0 {
        attacks |= bitboard >> 17
    };
    if ((bitboard >> 15) & NOT_A_FILE) > 0 {
        attacks |= bitboard >> 15
    };
    if ((bitboard >> 10) & NOT_HG_FILE) > 0 {
        attacks |= bitboard >> 10
    };
    if ((bitboard >> 6) & NOT_AB_FILE) > 0 {
        attacks |= bitboard >> 6
    };
    if ((bitboard << 17) & NOT_A_FILE) > 0 {
        attacks |= bitboard << 17
    };
    if ((bitboard << 15) & NOT_H_FILE) > 0 {
        attacks |= bitboard << 15
    };
    if ((bitboard << 10) & NOT_AB_FILE) > 0 {
        attacks |= bitboard << 10
    };
    if ((bitboard << 6) & NOT_HG_FILE) > 0 {
        attacks |= bitboard << 6
    };
    return attacks;
}
