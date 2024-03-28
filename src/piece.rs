use crate::board::Squares;

pub enum Sides {
    White,
    Black,
}

const NOT_A_FILE: u64 = 18374403900871474942;

// not H file constant
const NOT_H_FILE: u64 = 9187201950435737471;

// not HG file constant
const NOT_HG_FILE: u64 = 4557430888798830399;

// not AB file constant
const NOT_AB_FILE: u64 = 18229723555195321596;

// Mask Pawn i.e every attack move for squares
pub fn mask_pawn(sq: Squares, side: Sides) -> u64 {
    let attacks = 1 << sq as u8;

    // CHANGE THE BITBOARD TO ADD EVERY SQUARE THAT HAS
    // CHECK SIDE IF WHITE OR BLACK
    match side {
        Sides::White => {
            return (attacks & NOT_A_FILE) >> 7 | (attacks & NOT_H_FILE) >> 9
        }
        Sides::Black => {
            return (attacks & NOT_A_FILE) << 7 | (attacks & NOT_H_FILE) << 9
        }
    }
}

pub fn mask_knight(sq: Squares) -> u64 {
    let attacks = 1 << sq as u8;
     (attacks & NOT_A_FILE) >> 17
        | (attacks & NOT_A_FILE) << 15
        | (attacks & NOT_H_FILE) >> 15
        | (attacks & NOT_H_FILE) << 17
        | (attacks & NOT_AB_FILE) >> 10
        | (attacks & NOT_AB_FILE) << 6
        | (attacks & NOT_HG_FILE) >> 6
        | (attacks & NOT_HG_FILE) << 10
}

pub fn mask_king(sq: Squares) -> u64 {
    let attacks = 1 << sq as u8;
     (attacks >> 8 | attacks << 8)
        | (attacks & NOT_A_FILE) >> 9
        | (attacks & NOT_A_FILE) >> 1
        | (attacks & NOT_A_FILE) << 7
        | (attacks & NOT_H_FILE) >> 7
        | (attacks & NOT_H_FILE) << 1
        | (attacks & NOT_H_FILE) << 9
}

// Mask Bishop i.e every attack move for squares
pub fn mask_bishop(sq: Squares, bitboard: u64, block: u64) -> u64 {
    let mut attacks: u64 = 0;
    let tr = (sq as u8) / 8;
    let tf = (sq as u8) % 8;

    for &(dr, df) in &[(1, 1), (-1, 1), (1, -1), (-1, -1)] {
        let mut r = i32::from(tr) + i32::from(dr);
        let mut f = i32::from(tf) + i32::from(df);

        while r >= 1 && r <= 6 && f >= 1 && f <= 6 {
            attacks |= 1u64 << (r as u64 * 8 + f as u64);

            if ((1u64 << (r * 8 + f)) & block) > 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }
    attacks
  
}



pub fn mask_rook(sq: Squares) -> u64 {
    let mut attacks: u64 = 0;
    let tr = (sq as u8) / 8;
    let tf = (sq as u8) % 8;

    for r in (tr + 1)..=6 {
        attacks |= 1u64 << (u64::from(r) * 8 + u64::from(tf));
    }

    for r in (1..tr).rev() {
        attacks |= 1u64 << (u64::from(r) * 8 + u64::from(tf));
    }

    for f in (tf + 1)..=6 {
        attacks |= 1u64 << (u64::from(tr) * 8 + u64::from(f));
    }

    for f in 1..tf {
        attacks |= 1u64 << (u64::from(tr) * 8 + u64::from(f));
    }
    attacks
 }
