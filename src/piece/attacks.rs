use crate::position::{Bitboard, Sides, Squares};
use crate::defs::*;
use crate::utils::computed_magics::{BISHOP_MAGIC, ROOK_MAGICS};

pub fn mask_pawn(sq: Squares, side: Sides) -> u64 {
    let attacks = 1 << sq as u8;

    // CHANGE THE BITBOARD TO ADD EVERY SQUARE THAT HAS
    // CHECK SIDE IF WHITE OR BLACK
    match side {
        Sides::White => {
            return (attacks & NOT_A_FILE) >> 7 | (attacks & NOT_H_FILE) >> 9
        }
        Sides::Black => {
            return (attacks & NOT_A_FILE) << 7 | (attacks & NOT_H_FILE) << 9
        }
    }
}

pub fn mask_knight(sq: Squares) -> Bitboard {
    let attacks = 1 << sq as u8;
     (attacks & NOT_A_FILE) >> 17
        | (attacks & NOT_A_FILE) << 15
        | (attacks & NOT_H_FILE) >> 15
        | (attacks & NOT_H_FILE) << 17
        | (attacks & NOT_AB_FILE) >> 10
        | (attacks & NOT_AB_FILE) << 6
        | (attacks & NOT_HG_FILE) >> 6
        | (attacks & NOT_HG_FILE) << 10
}


pub fn mask_king(sq: Squares) -> Bitboard {
    let attacks = 1 << sq as u8;
     (attacks >> 8 | attacks << 8)
        | (attacks & NOT_A_FILE) >> 9
        | (attacks & NOT_A_FILE) >> 1
        | (attacks & NOT_A_FILE) << 7
        | (attacks & NOT_H_FILE) >> 7
        | (attacks & NOT_H_FILE) << 1
        | (attacks & NOT_H_FILE) << 9
}


/// Attacks for Slider pieces

/// Don't use this:  Compute Brute force attack for Bishop piece 
pub fn mask_bishop(sq: Squares, block: Bitboard) -> Bitboard {
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
}

/// Don't use this:  Compute Brute force attack for Rook piece 
pub fn mask_rook(sq: Squares) -> Bitboard {
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
 }

#[inline]
pub fn get_rook_attacks(square: Squares,occupancy:&u64,masks:&Vec<u64>,attacks:&Vec<u64>)->u64{
    let mut occupancy = occupancy &masks[square as usize]; 
    occupancy = occupancy.wrapping_mul(ROOK_MAGICS[square as usize]);
    occupancy >>= 64 - ROOK_REVEVANT_BITS[square as usize];
    attacks[square as usize * 4096 + occupancy as usize]
}


#[inline]
pub fn get_bishop_attacks(square: Squares,occupancy:&u64,masks:&Vec<u64>,attacks:&Vec<u64>)->u64{
    let mut occupancy = occupancy &masks[square as usize]; 
    occupancy = occupancy.wrapping_mul(BISHOP_MAGIC[square as usize]);
    occupancy >>= 64 - BISHOP_RELEVANT_BITS[square as usize];
    attacks[square as usize * 512 + occupancy as usize]
}


pub fn mask_queen(
    sq:Squares,
    occupancy:&Bitboard,
    b_mask:&Vec<Bitboard>,
    b_attacks:&Vec<Bitboard>,
    r_mask:&Vec<Bitboard>,
    r_attacks:&Vec<Bitboard>
    ) -> Bitboard {
        get_bishop_attacks(sq, occupancy, b_mask , b_attacks) |  get_rook_attacks(sq, occupancy, r_mask , r_attacks) 
}
