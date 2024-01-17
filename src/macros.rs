#[macro_export]
macro_rules! set_bit {
   

     ($squares:expr) => {
         { 
        let mut bitboard:u64 = 0; 
         for square in $squares {
            bitboard |=  1<< *square;          
         }
         bitboard 
         } 
    };

    ($squares:expr,mut $bitboard:expr)=>{
         {
         for square in $squares {
            $bitboard |=  1<< *square;          
         }
         $bitboard 
         } 
    };

}

#[macro_export]
macro_rules! pop_bit {
    ($square:expr,mut $bitboard:expr) => {
        match get_bit!($square,$bitboard) {
            1 => $bitboard^=(1<<$square),
            _ => {
            },
        };
    
}

}

#[macro_export]
macro_rules! get_bit {
    ($square:expr,$bitboard:expr) => {
        match $bitboard & (1 << $square) {
            0 => ".",
            _ => "1",
        }
    };
}

