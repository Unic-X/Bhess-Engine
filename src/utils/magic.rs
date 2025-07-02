use rand::prelude::random;
use strum::IntoEnumIterator;

use crate::piece::attacks::mask_bishop;
use crate::position::Squares;
use crate::{get_lsb, piece::attacks::mask_rook};

use crate::computed_magics::*;
use crate::defs::{BISHOP_RELEVANT_BITS, ROOK_REVEVANT_BITS};
use crate::utils::Slider;

/// Enumeration for Slider Pieces
/// Rook and Bishop

/// Bishop relevant occupancy bit count for every square on board

/// Magic numbers for the Rook

/// PRNG using bitshifts

pub fn get_random_64() -> u64 {
    let n1 = random::<u64>() & 0xFFFF;
    let n2 = random::<u64>() & 0xFFFF;
    let n3 = random::<u64>() & 0xFFFF;
    let n4 = random::<u64>() & 0xFFFF;

    n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
}

pub fn random_uint64_fewbits() -> u64 {
    get_random_64() & get_random_64() & get_random_64()
}

/// Generate attack for rook
/// https://www.chessprogramming.org/Magic_Bitboards

pub fn ratt(sq: u8, block: u64) -> u64 {
    let mut result = 0;
    let rk = (sq / 8) as u64;
    let fl = (sq % 8) as u64;

    for r in rk + 1..=7 {
        let index = fl + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    for r in (0..rk).rev() {
        let index = fl + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    for f in fl + 1..=7 {
        let index = f + rk * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    for f in (0..fl).rev() {
        let index = f + rk * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    result
}

///Generate attack for a bishop
///https://www.chessprogramming.org/Magic_Bitboards

pub fn batt(sq: u8, block: u64) -> u64 {
    let mut result = 0;
    let rk = (sq / 8) as u64;
    let fl = (sq % 8) as u64;

    for (r, f) in (rk + 1..=7).zip(fl + 1..=7) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    for (r, f) in (rk + 1..=7).zip((0..fl).rev()) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    for (r, f) in (0..rk).rev().zip(fl + 1..=7) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    for (r, f) in (0..rk).rev().zip((0..fl).rev()) {
        let index = f + r * 8;
        result |= 1 << index;
        if block & (1 << index) != 0 {
            break;
        }
    }

    result
}

/// Magic numbers can be found out by:
/// https://www.chessprogramming.org/Looking_for_Magics

pub fn find_magic(square: Squares, relevent_bits: u64, is_bishop: bool) -> u64 {
    let occupancy_index: usize = 1 << relevent_bits;
    let mut occupancies: Vec<u64> = vec![0; occupancy_index as usize];
    let mut attacks: Vec<u64> = vec![0; occupancy_index as usize];
    let mut used_attacks: Vec<u64> = vec![0; occupancy_index as usize];

    let mask: u64 = if is_bishop {
        mask_bishop(square, 0)
    } else {
        mask_rook(square)
    };

    for i in 0..occupancy_index {
        occupancies[i as usize] = set_occupancy(i as u64, relevent_bits, mask);

        attacks[i as usize] = if is_bishop {
            batt(square as u8, occupancies[i as usize])
        } else {
            ratt(square as u8, occupancies[i as usize])
        };
    }

    for _ in 0..1_000_0000_000 as usize {
        let magic_number = random_uint64_fewbits();
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
    return 0;
}

/// Generate Magic numbers!!
pub fn init_magic() {
    println!("BISHOP MAGIC NUMBERS:");
    for square in Squares::iter() {
        println!(
            "{} ,",
            find_magic(square, BISHOP_RELEVANT_BITS[square as usize].into(), true)
        );
    }
}

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

/// Generate attack tables for Slider piece (Rook, Bishop)
///
pub fn init_slider_attacks(piece: Slider) -> (Vec<u64>, Vec<u64>) {
    let mut attacks = match piece {
        Slider::Bishop => {
            vec![vec![0u64; 512]; 64]
        }
        Slider::Rook => {
            vec![vec![0u64; 4096]; 64]
        }
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
                    attacks[square as usize][magic_index as usize] = batt(square as u8, occupancy);
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
                    let magic_index =
                        occupancy.wrapping_mul(ROOK_MAGICS[idx]) >> (64 - ROOK_REVEVANT_BITS[idx]);
                    attacks[square as usize][magic_index as usize] = ratt(square as u8, occupancy);
                }
            }
        }
    }

    (attacks.into_iter().flatten().collect(), mask)
}
