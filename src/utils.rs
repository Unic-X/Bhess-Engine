use rand::prelude::*;

use crate::{board::*, piece::*};



#[inline]
pub fn get_lsb(bitboard:u64) ->Option<u64> {
    
   if bitboard>0{
        let _x = bitboard as i64;
        Some(bit_count((_x & -_x) as u64 - 1)) 
    }else{
        None
    } 
}

#[inline]
pub fn bit_count(bitboard:u64)->u64{
    
    let mut a = bitboard;
    let mut count:u64 = 0;
    while a>0 {
        count+=1;
        a&=a-1;  
    }  

    count

}
const BISHOP_RELEVANT_BITS: [u8;64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6
];

// rook relevant occupancy bit count for every square on board
const ROOK_REVEVANT_BITS: [u8;64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12
];



pub fn get_random_64()->u64{
    let n1 = random::<u64>() & 0xFFFF;
    let n2 = random::<u64>() & 0xFFFF;
    let n3 = random::<u64>() & 0xFFFF;
    let n4 = random::<u64>() & 0xFFFF;
   
    n1 | (n2 << 16) | (n3 << 32) | (n4 <<48)
}

pub fn random_uint64_fewbits()->u64 {
  get_random_64() & get_random_64() & get_random_64()
}



pub fn ratt(sq: u64, block: u64) -> u64 {
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

pub fn batt(sq: u64, block: u64) -> u64 {
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


pub fn find_magic(square:Squares, relevent_bits:u64,is_bishop:bool)->u64{
    let mut occupancies:Vec<u64> = vec![0; 1 << 12];
    let mut attacks:Vec<u64> = vec![0; 1 << 12];
    let used_attacks:Vec<u64> = vec![0; 1 << 12];

    let mask:u64;
   
    if is_bishop {
        mask = mask_bishop(square, 0, 0);
    }else {
        mask = mask_rook(square, 0);
    }
    let n = bit_count(mask); 
    for i in 0..(1 << n) {
        occupancies[i] = set_occupancy(i as u64, relevent_bits, mask);
        attacks[i] = if is_bishop { batt(square as u64, occupancies[i]) } else { ratt(square as u64, occupancies[i]) };
    }

    for _k in 0..100000000 {
        let magic_number = random_uint64_fewbits();
        if bit_count(mask*magic_number) & 0xFF00000000000000 < 6 {
            println!("maa chuda");
            continue;
        }
        
    

    }
    return 1;
}

pub fn set_occupancy(index: u64, bits_in_mask: u64, attack_mask: u64) -> u64 {
    let mut occupancy: u64 = 0;
    let mut current_attack_mask = attack_mask;

    for count in 0..bits_in_mask {
        let square = get_lsb(current_attack_mask);
        current_attack_mask &= !(1 << square.unwrap());
        
        if (index & (1 << count)) != 0 {
            occupancy |= 1 << square.unwrap();
        }
    }

    occupancy
}
