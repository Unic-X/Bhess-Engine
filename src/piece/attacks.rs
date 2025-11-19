use crate::defs::*;
use crate::position::{Bitboard, Sides, Squares};
use crate::utils::computed_magics::{BISHOP_MAGIC, ROOK_MAGICS};


/// Returns pawn attack bitboard for a given square and side.
///
/// This generates *capture attacks only*, not forward pushes.
/// It masks out illegal attacks at board edges using file-exclusion masks.
///
/// # Arguments
/// - `sq`: The pawn's square (0–63)
/// - `side`: `Sides::White` or `Sides::Black`
///
/// # Returns
/// Bitboard of squares the pawn can attack.
///
/// # Notes
/// - White attacks up (toward rank 8): `>>7`, `>>9`
/// - Black attacks down (toward rank 1): `<<7`, `<<9`
/// - Uses `NOT_A_FILE` / `NOT_H_FILE` to prevent wraparound.
pub fn mask_pawn(sq: Squares, side: Sides) -> u64 {
    let attacks = 1 << sq as u8;

    match side {
        Sides::White => (attacks >> 7 & NOT_A_FILE) | (attacks >> 9 & NOT_H_FILE),
        Sides::Black => (attacks << 7 & NOT_H_FILE) | (attacks << 9 & NOT_A_FILE),
    }
}



/// Returns knight attack bitboard for a given square.
///
/// Uses bit shifts with file-exclusion masks to avoid wraparound.
/// Covers all 8 possible L-shaped knight moves.
///
/// # Arguments
/// - `sq`: Knight square
///
/// # Returns
/// Bitboard of knight attacks.
///
/// # Reference
/// <https://www.chessprogramming.org/Knight_Pattern>
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



/// Returns king attack bitboard for a given square.
///
/// Generates all 8 adjacent-square moves using shifts and
/// file-exclusion masks to prevent wraparound.
///
/// # Arguments
/// - `sq`: King square
///
/// # Returns
/// Bitboard of king attacks.
///
/// # Reference
/// <https://www.chessprogramming.org/King_Pattern>
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



/// Brute-force bishop attack generator used for magic bitboards.
///
/// Walks diagonally in all four directions until:
/// - the ray leaves the *inner* 6×6 board region (for magic mask generation)
/// - a blocker bit is hit
///
/// # Arguments
/// - `sq`: Bishop square
/// - `block`: Occupancy bitboard
///
/// # Returns
/// Bitboard of bishop attacks (**not** including edges if restricted mask is intended)
///
/// # Notes
/// This version intentionally restricts scanning to ranks/files `1..6`
/// because it is used to build bishop *attack masks* (not true attacks).
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



/// Brute-force rook attack mask generator used for magic bitboards.
///
/// Generates sliding moves in four straight directions, stopping when the
/// ray exits the *inner* 6×6 region. This is used only for generating
/// occupancy masks for magic indexing.
///
/// # Arguments
/// - `sq`: Rook square
///
/// # Returns
/// Bitboard of rook mask squares.
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



/// Returns rook magic-bitboard attacks for a given square and occupancy.
///
/// Computes:
/// 1. Relevant occupancy via `& masks[square]`  
/// 2. Magic multiplication  
/// 3. Right-shift of unused bits  
/// 4. Lookup into precomputed attack table  
///
/// # Arguments
/// - `square`: Rook square
/// - `occupancy`: Board occupancy
/// - `masks`: Rook occupancy masks (size 64)
/// - `attacks`: Flattened rook attack table (64 × 4096 entries)
///
/// # Returns
/// Bitboard of rook attacks.
///
/// # Reference
/// <https://www.chessprogramming.org/Magic_Bitboards>
#[inline]
pub fn get_rook_attacks(
    square: Squares,
    occupancy: &u64,
    masks: &Vec<u64>,
    attacks: &Vec<u64>,
) -> u64 {
    let mut occupancy = occupancy & masks[square as usize];
    occupancy = occupancy.wrapping_mul(ROOK_MAGICS[square as usize]);
    occupancy >>= 64 - ROOK_REVEVANT_BITS[square as usize];
    attacks[square as usize * 4096 + occupancy as usize]
}



/// Returns bishop magic-bitboard attacks for a given square and occupancy.
///
/// Computation is identical to rook logic but uses bishop-specific:
/// - masks
/// - magic constants
/// - table size (`512` per square)
///
/// # Arguments
/// - `square`: Bishop square
/// - `occupancy`: Board occupancy
/// - `masks`: Bishop occupancy masks (size 64)
/// - `attacks`: Flattened bishop attack table (64 × 512 entries)
///
/// # Returns
/// Bitboard of bishop attacks.
#[inline]
pub fn get_bishop_attacks(
    square: Squares,
    occupancy: &u64,
    masks: &Vec<u64>,
    attacks: &Vec<u64>,
) -> u64 {
    let mut occupancy = occupancy & masks[square as usize];
    occupancy = occupancy.wrapping_mul(BISHOP_MAGIC[square as usize]);
    occupancy >>= 64 - BISHOP_RELEVANT_BITS[square as usize];
    attacks[square as usize * 512 + occupancy as usize]
}



/// Returns queen attacks by combining bishop + rook attacks using magic bitboards.
///
/// # Arguments
/// - `sq`: Queen square
/// - `occupancy`: Board occupancy
/// - `b_mask`: Bishop occupancy masks
/// - `b_attacks`: Bishop attack table
/// - `r_mask`: Rook occupancy masks
/// - `r_attacks`: Rook attack table
///
/// # Returns
/// Bitboard of queen attacks.
///
/// # Notes
/// A queen = bishop OR rook attacks.
pub fn mask_queen(
    sq: Squares,
    occupancy: &Bitboard,
    b_mask: &Vec<Bitboard>,
    b_attacks: &Vec<Bitboard>,
    r_mask: &Vec<Bitboard>,
    r_attacks: &Vec<Bitboard>,
) -> Bitboard {
    get_bishop_attacks(sq, occupancy, b_mask, b_attacks)
        | get_rook_attacks(sq, occupancy, r_mask, r_attacks)
}