use rand::prelude::random;
use strum::IntoEnumIterator;

use crate::piece::attacks::mask_bishop;
use crate::position::Squares;
use crate::{get_lsb, piece::attacks::mask_rook};

use crate::computed_magics::*;
use crate::defs::{BISHOP_RELEVANT_BITS, ROOK_REVEVANT_BITS};
use crate::utils::Slider;


/// Returns a pseudo random 64-bit integer with roughly uniform distribution.
/// Uses four 16-bit chunks combined into a full u64.
///
/// This is used by the magic bitboard generator when searching
/// for candidate magic numbers.
///
/// # Reference
/// - <https://www.chessprogramming.org/Random_Number_Generators>
fn get_random_64() -> u64 {
    let n1 = random::<u64>() & 0xFFFF;
    let n2 = random::<u64>() & 0xFFFF;
    let n3 = random::<u64>() & 0xFFFF;
    let n4 = random::<u64>() & 0xFFFF;

    n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
}


/// Returns a pesudo random 64-bit integer with very few set bits (sparse).
/// This helps find magic numbers that reduce collisions when indexing.
///
/// # Reference
/// - <https://www.chessprogramming.org/Magic_Bitboards#Sparse>
fn random_uint64_fewbits() -> u64 {
    get_random_64() & get_random_64() & get_random_64()
}


/// Computes rook sliding attacks on an empty or partially blocked board.
/// This is the *brute force* move generator used during magic lookup table creation.
/// Not used at runtime.
///
/// # Arguments
/// - `sq`: square index (0–63)
/// - `block`: occupancy bitboard (1 = blocking piece)
///
/// # Returns
/// A bitboard of rook attacks from `sq`.
///
/// # Reference
/// - <https://www.chessprogramming.org/Rook_Attacks>
fn ratt(sq: u8, block: u64) -> u64 {
    let mut result = 0;
    let rk = (sq / 8) as u64;
    let fl = (sq % 8) as u64;

    // Up
    for r in rk + 1..=7 {
        let index = fl + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    // Down
    for r in (0..rk).rev() {
        let index = fl + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    // Right
    for f in fl + 1..=7 {
        let index = f + rk * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    // Left
    for f in (0..fl).rev() {
        let index = f + rk * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    result
}


/// Computes bishop sliding attacks on an empty or partially blocked board.
/// This is the *brute force* reference generator used by the magic table builder.
///
/// # Arguments
/// - `sq`: square index (0–63)
/// - `block`: occupancy bitboard
///
/// # Returns
/// A bitboard of bishop attacks.
///
/// # Reference
/// - <https://www.chessprogramming.org/Bishop_Attacks>
fn batt(sq: u8, block: u64) -> u64 {
    let mut result = 0;
    let rk = (sq / 8) as u64;
    let fl = (sq % 8) as u64;

    // Up-right
    for (r, f) in (rk + 1..=7).zip(fl + 1..=7) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 { break; }
    }

    // Up-left
    for (r, f) in (rk + 1..=7).zip((0..fl).rev()) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 { break; }
    }

    // Down-right
    for (r, f) in (0..rk).rev().zip(fl + 1..=7) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 { break; }
    }

    // Down-left
    for (r, f) in (0..rk).rev().zip((0..fl).rev()) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 { break; }
    }

    result
}


/// Attempts to find a magic number for a given square.
/// The function randomly searches for a number that maps all occupancies
/// to *unique* attack sets using perfect hashing.
///
/// This is slow and should only be run to generate magic constants once.
///
/// # Arguments
/// - `square`: board square (enum)
/// - `relevent_bits`: number of occupancy bits in the mask
/// - `is_bishop`: true → bishop, false → rook
///
/// # Returns
/// A valid magic number, or `0` if not found (unlikely).
///
/// # Reference
/// - <https://www.chessprogramming.org/Looking_for_Magics>
fn find_magic(square: Squares, relevent_bits: u64, is_bishop: bool) -> u64 {
    let occupancy_index: usize = 1 << relevent_bits;
    let mut occupancies: Vec<u64> = vec![0; occupancy_index];
    let mut attacks: Vec<u64> = vec![0; occupancy_index];
    let mut used_attacks: Vec<u64> = vec![0; occupancy_index];

    let mask: u64 = if is_bishop {
        mask_bishop(square, 0)
    } else {
        mask_rook(square)
    };

    // Generate all possible occupancies and their corresponding attack sets.
    for i in 0..occupancy_index {
        occupancies[i] = set_occupancy(i as u64, relevent_bits, mask);
        attacks[i] = if is_bishop {
            batt(square as u8, occupancies[i])
        } else {
            ratt(square as u8, occupancies[i])
        };
    }

    // Try random sparse magic numbers
    for _ in 0..1_000_000_000 {
        let magic_number = random_uint64_fewbits();

        // Optional heuristic: magic must have enough high bits
        if u64::count_ones(mask.wrapping_mul(magic_number) & 0xFF00000000000000) < 6 {
            continue;
        }

        used_attacks.iter_mut().for_each(|a| *a = 0);
        let mut fail = false;

        for i in 0..occupancy_index {
            let j = (occupancies[i].wrapping_mul(magic_number) >> (64 - relevent_bits)) as usize;
            if used_attacks[j] == 0 {
                used_attacks[j] = attacks[i];
            } else if used_attacks[j] != attacks[i] {
                fail = true;
                break;
            }
        }

        if !fail {
            return magic_number;
        }
    }
    0
}


/// Generates and prints bishop magic numbers for all 64 squares.
/// This should only be run offline to regenerate the `BISHOP_MAGIC` array.
pub fn init_magic() {
    println!("BISHOP MAGIC NUMBERS:");
    for square in Squares::iter() {
        println!(
            "{} ,",
            find_magic(square, BISHOP_RELEVANT_BITS[square as usize].into(), true)
        );
    }

    println!("ROOK MAGIC NUMBERS:");
    for square in Squares::iter() {
        println!(
            "{} ,",
            find_magic(square, ROOK_REVEVANT_BITS[square as usize].into(), false)
        );
    }
}


/// Generates a bitboard occupancy using an index, relevant bits, and mask.
/// This is used during magic number testing to enumerate all possible
/// blocker configurations.
///
/// # Arguments
/// - `index`: index between `0` and `(1 << bits_in_mask) - 1`
/// - `bits_in_mask`: number of relevant squares
/// - `attack_mask`: mask containing sliding ray squares
///
/// # Returns
/// A correctly mapped occupancy bitboard.
pub fn set_occupancy(index: u64, bits_in_mask: u64, attack_mask: u64) -> u64 {
    let mut occupancy: u64 = 0;
    let mut current_attack_mask = attack_mask;

    for count in 0..bits_in_mask {
        let square = get_lsb!(current_attack_mask);
        current_attack_mask &= !(1 << square.unwrap());

        if (index & (1 << count)) != 0 {
            occupancy |= 1 << square.unwrap();
        }
    }

    occupancy
}


/// Builds the full magic attack table for either rook or bishop.
/// This is run once at engine startup.
///
/// # Arguments
/// - `piece`: Slider::Rook or Slider::Bishop
///
/// # Returns
/// `(flattened_attack_table, masks)`
///
/// The attack table is returned flattened for efficient indexing.
///
/// # Reference
/// - <https://www.chessprogramming.org/Magic_Bitboards#Initialization>
pub fn init_slider_attacks(piece: Slider) -> (Vec<u64>, Vec<u64>) {
    let mut attacks = match piece {
        Slider::Bishop => vec![vec![0u64; 512]; 64],
        Slider::Rook => vec![vec![0u64; 4096]; 64],
    };

    let mut mask: Vec<u64> = Vec::new();

    match piece {
        Slider::Bishop => {
            for (idx, square) in Squares::iter().enumerate() {
                let attack_mask = mask_bishop(square, 0);
                mask.push(attack_mask);
                let relevent_bit_count = attack_mask.count_ones() as u64;

                let occupancy_indices = 1 << relevent_bit_count;
                for index in 0..occupancy_indices {
                    let occupancy = set_occupancy(index, relevent_bit_count, attack_mask);
                    let magic_index = occupancy.wrapping_mul(BISHOP_MAGIC[idx])
                        >> (64 - BISHOP_RELEVANT_BITS[idx]);
                    attacks[square as usize][magic_index as usize] =
                        batt(square as u8, occupancy);
                }
            }
        }

        Slider::Rook => {
            for (idx, square) in Squares::iter().enumerate() {
                let attack_mask = mask_rook(square);
                mask.push(attack_mask);
                let relevent_bit_count = attack_mask.count_ones() as u64;

                let occupancy_indices = 1 << relevent_bit_count;
                for index in 0..occupancy_indices {
                    let occupancy = set_occupancy(index, relevent_bit_count, attack_mask);
                    let magic_index = occupancy.wrapping_mul(ROOK_MAGICS[idx])
                        >> (64 - ROOK_REVEVANT_BITS[idx]);
                    attacks[square as usize][magic_index as usize] =
                        ratt(square as u8, occupancy);
                }
            }
        }
    }

    (attacks.into_iter().flatten().collect(), mask)
}
