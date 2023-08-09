macro_rules! get_bit {
    ($s:expr) => {};
}

pub fn display(bitboard: u64) {
    //loop over ranks
    for rank in 0..8 {
        for file in 0..8 {
            //Use ranks and file to convert into Square number
            let square = rank * 8 + file;
            match bitboard & (1 << square) {
                0 => print!("0"),
                _ => print!("1"),
            }
        }
        print!("\n");
    }
}
