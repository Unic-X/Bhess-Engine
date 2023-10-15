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


// Mask Pawn i.e every attack move for squares
pub fn mask_pawn(sq: &[Squares], side: Sides, mut bitboard: u64) -> u64 {
    let mut attacks: u64 = 0;


    // CHANGE THE BITBOARD TO ADD EVERY SQUARE THAT HAS 
    set_bit!(sq, mut bitboard);
    // CHECK SIDE IF WHITE OR BLACK 
    match side {
        Sides::White => {
            if ((bitboard >> 7) & NOT_A_FILE) > 0 {
                attacks |= bitboard >> 7;
            } 
            if ((bitboard >> 9) & NOT_H_FILE) > 0 {
                attacks |= bitboard >> 9;
            }
        }
        Sides::Black => {
            if ((bitboard << 7) & NOT_H_FILE) > 0 {
                attacks |= bitboard << 7;
            }
            if ((bitboard << 9) & NOT_A_FILE) > 0 {
                attacks |= bitboard << 9;
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

pub fn mask_king(sq: &[Squares], mut bitboard: u64) -> u64 {
    let mut attacks: u64 = 0;

    set_bit!(sq, mut bitboard);

    if (bitboard >> 8) > 0 {
        attacks |= bitboard >> 8
    };

    if ((bitboard >> 9) & NOT_H_FILE) > 0 {
        attacks |= bitboard >> 9
    };
    if ((bitboard >> 7) & NOT_A_FILE) > 0 {
        attacks |= bitboard >> 7
    };
    if ((bitboard >> 1) & NOT_H_FILE) > 0 {
        attacks |= bitboard >> 1
    };

    if (bitboard << 8) > 0 {
        attacks |= bitboard << 8
    };

    if ((bitboard << 9) & NOT_A_FILE) > 0 {
        attacks |= bitboard << 9
    };
    if ((bitboard << 7) & NOT_H_FILE) > 0 {
        attacks |= bitboard << 7
    };
    if ((bitboard << 1) & NOT_A_FILE) > 0 {
        attacks |= bitboard << 1
    };
    return attacks;
}




// Mask Pawn i.e every attack move for squares
/*pub fn mask_bishop(sq: Squares, side: Sides, mut bitboard: u64) -> u64 {
    let mut attacks: u64 = 0;
    let tr = (sq as u8)  / 8;
    let tf = (sq as u8) % 8;

    let r=0;let f = 0;
    while  {
        unimplemented!();
    }

    for (r = tr + 1, f = tf + 1; r <= 6 && f <= 6; r++, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf + 1; r >= 1 && f <= 6; r--, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr + 1, f = tf - 1; r <= 6 && f >= 1; r++, f--) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf - 1; r >= 1 && f >= 1; r--, f--) attacks |= (1ULL << (r * 8 + f));
}*/


