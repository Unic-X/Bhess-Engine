
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


const ROOK_MAGICS:[u64;64]= [
   4719772841163636736 ,
    9295447777131642886 ,
    6989621807125954569 ,
    3819074611950151936 ,
    684549359566061696 ,
    4683748010545447048 ,
    612500544723878016 ,
    144119587204562994 ,
    23784774044778496 ,
    1180506194066933828 ,
    1171358184319746176 ,
    578853358325465218 ,
    289919303323485192 ,
    13792286744265728 ,
    9548897856010649728 ,
    288793328270459428 ,
    6958114750743986176 ,
    887210502055272450 ,
    13835200992868696096 ,
    36170084531570688 ,
    1153062791918127104 ,
    577586927121744128 ,
    649363323174846976 ,
    1171162402584035588 ,
    18016221723377792 ,
    2305983888440164609 ,
    2310351007426545664 ,
    9223389633336377376 ,
    2305922184788574336 ,
    4785087491342344 ,
    648535955709526280 ,
    2323866487284303491 ,
    2305913380113743920 ,
    4503737070534656 ,
    563089573417024 ,
    290218297486477344 ,
    144255959932339200 ,
    166637586415092224 ,
    1161929872159803586 ,
    140876009443584 ,
    11294768629841920 ,
    288265561865994240 ,
    4538784267927680 ,
    9259437118025957384 ,
    2251938326643712 ,
    1407383540662280 ,
    651333171409977348 ,
    9223372659629883404 ,
    1154122890712514816 ,
    1157495499285172352 ,
    140875012244608 ,
    2326126799892971776 ,
    1126449796908160 ,
    5067099403715200 ,
    1243011123733693440 ,
    288797811411026432 ,
    180869700522241 ,
    288300781404168449 ,
    12455405192675345 ,
    288511919850047525 ,
    4900479361982924802 ,
    180707003835351298 ,
    13528468931258372 ,
    12826257237653881862, 
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
    19778170450479105 ,
    1299289815757492480 ,
    41096463845163521 ,
    1731636264488241153 ,
    9011670391719009 ,
    35802849511428 ,
    586031040024609026 ,
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

///Generate attack for bishop
///https://www.chessprogramming.org/Magic_Bitboards

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
        mask_rook(square, 0)
    };
        
    
    for i in 0..occupancy_index {
        occupancies[i as usize] = set_occupancy(i as u64, relevent_bits, mask);
       
        attacks[i as usize] = if is_bishop { batt(square as u64, occupancies[i as usize]) } else { ratt(square as u64, occupancies[i as usize]) };
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
