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
pub fn mask_bishop(sq: Squares, mut bitboard: u64, block: u64) -> u64 {
    let mut attacks: u64 = 0;
    let tr = (sq as u8) / 8;
    let tf = (sq as u8) % 8;

    for &(dr, df) in &[(1, 1), (-1, 1), (1, -1), (-1, -1)] {
        let mut r = i32::from(tr) + i32::from(dr);
        let mut f = i32::from(tf) + i32::from(df);

        while r >= 1 && r <= 6 && f >= 1 && f <= 6 {
            attacks |= 1u64 << (r as u64 * 8 + f as u64);

            if ((1u64 << (r * 8 + f)) & block) > 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }
    attacks
    /*for (r = tr + 1, f = tf + 1; r <= 6 && f <= 6; r++, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf + 1; r >= 1 && f <= 6; r--, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr + 1, f = tf - 1; r <= 6 && f >= 1; r++, f--) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf - 1; r >= 1 && f >= 1; r--, f--) attacks |= (1ULL << (r * 8 + f));
    */
}

pub fn mask_rook(sq: Squares, mut bitboard: u64) -> u64 {
    let mut attacks: u64 = 0;
    let tr = (sq as u8) / 8;
    let tf = (sq as u8) % 8;

    for r in (tr + 1)..=6 {
        attacks |= 1u64 << (u64::from(r) * 8 + u64::from(tf));
    }

    for r in (1..tr).rev() {
        attacks |= 1u64 << (u64::from(r) * 8 + u64::from(tf));
    }

    for f in (tf + 1)..=6 {
        attacks |= 1u64 << (u64::from(tr) * 8 + u64::from(f));
    }

    for f in 1..tf {
        attacks |= 1u64 << (u64::from(tr) * 8 + u64::from(f));
    }
    attacks
    /*for (r = tr + 1, f = tf + 1; r <= 6 && f <= 6; r++, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf + 1; r >= 1 && f <= 6; r--, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr + 1, f = tf - 1; r <= 6 && f >= 1; r++, f--) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf - 1; r >= 1 && f >= 1; r--, f--) attacks |= (1ULL << (r * 8 + f));
    */
}
