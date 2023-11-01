

pub fn bit_count(bitboard:u64)->u32{
    
    let mut a = bitboard;
    let mut count:u32 = 0;
    while a>0 {
        count+=1;
        a&=a-1;  
    }  

    count

}
