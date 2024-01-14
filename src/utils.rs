
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{board::*, piece::*};

#[inline]
pub fn get_lsb(bitboard:u64) ->Option<u64> {
    
   if bitboard>0{
        let _x = bitboard as i64;
        Some(u64::count_ones((_x & -_x) as u64 - 1) as u64) 
    }else{
        None
    } 
}


pub enum Slider {
    Rook,
    Bishop,
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



const ROOK_MAGICS: [u64; 64] = [
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
];



const BISHOP_MAGIC:[u64;64] = [
    4620993457387995267 ,
    81242931576446976 ,
    2429972272644096 ,
    1162105194805481472 ,
    10156326495847432 ,
    2323302970867736 ,
    576752161828913152 ,
    140877215303684 ,
    18033142099608070 ,
    37393126402178 ,
    4616198422744137748 ,
    18402116120870912 ,
    2306973324900631104 ,
    3026420066830018562 ,
    9547913252493594664 ,
    1301540568329101312 ,
    2380227187577455104 ,
    1130306619315712 ,
    282033456808068 ,
    2314889799613350032 ,
    41095364318658560 ,
    3459328567586456064 ,
    2459106151355420672 ,
    221241677302536193 ,
    4612284187937083464 ,
    4508001029407746 ,
    2254001001464064 ,
    12948398818754562 ,
    281543712981001 ,
    5190857066994010752 ,
    2267193518134272 ,
    9799905357605143562 ,
    316865509397504 ,
    6917687391675549696 ,
    145139830884425 ,
    2918351284593754177 ,
    18015502316081216 ,
    45036326987268228 ,
    4794149870373120 ,
    3458909656133420544 ,
    24779163932042242 ,
    9082258170585104 ,
    441007309064202 ,
    19140573596487744 ,
    286216763286528 ,
    24787391243881106 ,
    572029581787264 ,
    11677908602986696737 ,
    1157566425976537099 ,
    649121437330728960 ,
    9225624128793616464 ,
    2305843026939346945 ,
    9223513050068946944 ,
    9051194349977600 ,
    1299289815757492480 ,
    41096463845163521 ,
    1731636264488241153 ,
    9011670391719009 ,
    35802849511428 ,
    586031040024609026 ,
    19778170450479105 ,
    1155182375442055425 ,
    721417201031306 ,
    9521753447987159232 ,
];




/// rook relevant occupancy bit count for every square on board
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

//PRNG using bitshifts 
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



///Generate attack for rook
///https://www.chessprogramming.org/Magic_Bitboards

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

///Generate attack for bishop
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


///Magic numbers can be found out in 
///https://www.chessprogramming.org/Looking_for_Magics

pub fn find_magic(square:Squares, relevent_bits:u64,is_bishop:bool)->u64{

    let occupancy_index:usize= 1<<relevent_bits;
    let mut occupancies:Vec<u64> = vec![0; occupancy_index as usize];
    let mut attacks:Vec<u64> = vec![0;occupancy_index as usize];
    let mut used_attacks:Vec<u64> = vec![0; occupancy_index as usize];

    let mask:u64 = if is_bishop {
        mask_bishop(square, 0, 0)
    }else {
        mask_rook(square)
    };
        
    
    for i in 0..occupancy_index {
        occupancies[i as usize] = set_occupancy(i as u64, relevent_bits, mask);
       
        attacks[i as usize] = if is_bishop { batt(square as u8, occupancies[i as usize]) } else { ratt(square as u8, occupancies[i as usize]) };
    }

    for _ in 0..10_0000_000{ 
        let magic_number = random_uint64_fewbits();
        if u64::count_ones(mask.wrapping_mul(magic_number) & 0xFF00000000000000 ) < 6 {
            continue;
        }
        used_attacks.iter_mut().for_each(|a| *a=0);
        let mut fail = false;
        for i in 0..occupancy_index {
            let j= (occupancies[i].wrapping_mul(magic_number)>>(64-relevent_bits))as usize;
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



///Generate Magic numbers!!
pub fn init_magic(){
    println!("BISHOP MAGIC NUMBERS:");
    for square in Squares::iter(){
         println!("{} , {:?}",find_magic(square, BISHOP_RELEVANT_BITS[square as usize].into(), true),square); 
    }
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

pub fn get_rook_attacks(square: Squares,occupancy:u64,masks:Vec<u64>,attacks:Vec<Vec<u64>>)->u64{
    let mut occupancy = occupancy &masks[square as usize]; 
    occupancy = occupancy.wrapping_mul(ROOK_MAGICS[square as usize]);
    occupancy >>= 64 - ROOK_REVEVANT_BITS[square as usize];
    attacks[square as usize][occupancy as usize]
}



pub fn get_bishop_attacks(square: Squares,occupancy:u64,masks:Vec<u64>,attacks:Vec<Vec<u64>>)->u64{
    let mut occupancy = occupancy &masks[square as usize]; 
    occupancy = occupancy.wrapping_mul(BISHOP_MAGIC[square as usize]);
    occupancy >>= 64 - BISHOP_RELEVANT_BITS[square as usize];
    attacks[square as usize][occupancy as usize]
}



pub fn init_slider_attacks(piece:Slider)->(Vec<Vec<u64>>,Vec<u64>){
    let mut attacks = match piece {
        Slider::Bishop => {
            vec![vec![0u64;512];64]
        },
        Slider::Rook =>{
            vec![vec![0u64;4096];64]
        }
    };
    let mut mask:Vec<u64> = Vec::new();
    match piece {
        Slider::Bishop => {
            for (idx,square) in Squares::iter().enumerate(){
                let attack_mask = mask_bishop(square, 0,0);
                mask.push(attack_mask);
                let relevent_bit_count = attack_mask.count_ones() as u64;

                let occupancy_indices = 1 << relevent_bit_count;
                for index in 0..occupancy_indices {
                    let occupancy = set_occupancy(index, relevent_bit_count, attack_mask);
                    let magic_index = occupancy.wrapping_mul(BISHOP_MAGIC[idx]) >> (64 - BISHOP_RELEVANT_BITS[idx]);
                    attacks[square as usize][magic_index as usize] = batt(square as u8, occupancy);
                }
            }

        },
        Slider::Rook => {
           for (idx,square) in Squares::iter().enumerate(){
                let attack_mask = mask_rook(square);
                mask.push(attack_mask);
                let relevent_bit_count = attack_mask.count_ones() as u64;

                let occupancy_indices = 1 << relevent_bit_count;
                for index in 0..occupancy_indices {
                    let occupancy = set_occupancy(index, relevent_bit_count, attack_mask);
                    let magic_index = occupancy.wrapping_mul(ROOK_MAGICS[idx]) >> (64 - ROOK_REVEVANT_BITS[idx]);
                    attacks[square as usize][magic_index as usize] = ratt(square as u8, occupancy);
                }
            }

        },
    }


    (attacks,mask)
}

