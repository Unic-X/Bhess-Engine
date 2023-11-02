
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


pub fn set_occupancy(index: u64, bits_in_mask: u32, attack_mask: u64) -> u64 {
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
