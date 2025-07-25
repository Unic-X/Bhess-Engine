/// Set the bit for the bitboard the macro defines two
/// ways to set the bitboard
/// 1. Set using vec of squares on the empty bitboard
/// ```
/// set_bit!(vec![Squares::e3,Squares::d2]);
/// ```
/// 2. Set using squares on existing bitboard this may mutate the bitboard
/// ```
/// set_bit!(vec![Squares::e3,Squares::d2],1020200102020);
/// ```

#[macro_export]
macro_rules! set_bit {
    ($squares:expr) => {{
        let mut bitboard: u64 = 0;
        for square in $squares {
            bitboard |= 1 << square;
        }
        bitboard
    }};

    ($square:expr, $bitboard:expr) => {{
        $bitboard |= 1 << $square;
        $bitboard
    }};

    ($squares:expr, $bitboard:expr) => {{
        for square in $squares {
            $bitboard |= 1 << square;
        }
        $bitboard
    }};

    ($squares:expr, mut $bitboard:expr) => {{
        for square in $squares {
            $bitboard |= 1 << *square;
        }
        $bitboard
    }};
}

#[macro_export]
macro_rules! pop_bit {
    ($square:expr,mut $bitboard:expr) => {
        match get_bit!($square, $bitboard) {
            1 => $bitboard ^= (1 << $square),
            _ => {}
        };
    };
}

#[macro_export]
macro_rules! get_bit {
    ($square:expr,$bitboard:expr) => {
        match $bitboard & (1 << $square) {
            0 => false,
            _ => true,
        }
    };
}

#[macro_export]
macro_rules! get_printable {
    ($square:expr,$bitboard:expr) => {
        match $bitboard & (1 << $square) {
            0 => '.',
            _ => '1',
        }
    };
}

#[macro_export]
macro_rules! get_lsb {
    ($bitboard:expr) => {
        if $bitboard > 0 {
            let _x = $bitboard as i64;
            Some(u64::count_ones((_x & -_x) as u64 - 1) as u64)
        } else {
            None
        }
    };
}
