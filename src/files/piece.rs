use std::ops::BitOr;

use strum_macros::EnumIter;

use crate::files::board::*;

use super::utils::{get_rook_attacks, get_bishop_attacks};

#[derive(PartialEq, Eq,Hash)]
pub enum Sides {
    White,
    Black,
}

/// Types of Pieces
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter,Hash)]
pub enum PieceKind{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl PieceKind {
    pub fn index(self) -> usize{
        match self {
            PieceKind::King => 0,
            PieceKind::Queen => 1,
            PieceKind::Rook => 2,
            PieceKind::Knight => 3,
            PieceKind::Bishop => 4,
            PieceKind::Pawn => 5,
        }
    }
}
#[derive(PartialEq,Eq,Hash)]
pub struct Piece{
    pub color : Sides,
    pub kind : PieceKind,
}


impl Piece {
    pub fn index(&self) -> usize{
        match (&self.color, &self.kind) {
            (Sides::White, PieceKind::Pawn ) => 0,
            (Sides::White, PieceKind::Knight ) => 1,
            (Sides::White, PieceKind::Bishop) =>  2,
            (Sides::White, PieceKind::Rook) => 3,
            (Sides::White, PieceKind::Queen) => 4,
            (Sides::White, PieceKind::King) => 5,

            (Sides::Black, PieceKind::Pawn ) => 6,
            (Sides::Black, PieceKind::Knight) => 7,
            (Sides::Black, PieceKind::Bishop) => 8,
            (Sides::Black, PieceKind::Rook) => 9,
            (Sides::Black, PieceKind::Queen) => 10,
            (Sides::Black, PieceKind::King) => 11,
        }
    }
        
    pub const fn pawn(color: Sides) -> Self {
        Self { kind: PieceKind::Pawn, color }
    }

    pub const fn knight(color: Sides) -> Self {
        Self { kind: PieceKind::Knight, color }
    }

    pub const fn bishop(color: Sides) -> Self {
        Self { kind: PieceKind::Bishop, color }
    }

    pub const fn rook(color:  Sides)-> Self {
        Self { kind: PieceKind::Rook, color }
    }

    pub const fn queen(color: Sides) -> Self {
        Self { kind: PieceKind::Queen, color }
    }

    pub const fn king(color: Sides) -> Self {
        Self { kind: PieceKind::King, color }
    }

    // Get the "fancy" character for this piece
    pub fn fancy_char(&self) -> &'static str {
        match (&self.color, &self.kind) {
            (Sides::White, PieceKind::Pawn ) => "♙",
            (Sides::White, PieceKind::Knight ) => "♘",
            (Sides::White, PieceKind::Bishop) =>  "♗",
            (Sides::White, PieceKind::Rook) => "♖",
            (Sides::White, PieceKind::Queen) => "♕",
            (Sides::White, PieceKind::King) => "♔",
            (Sides::Black, PieceKind::Pawn ) => "♟",
            (Sides::Black, PieceKind::Knight) => "♞",
            (Sides::Black, PieceKind::Bishop) => "♝",
            (Sides::Black, PieceKind::Rook) => "♜",
            (Sides::Black, PieceKind::Queen) => "♛",
            (Sides::Black, PieceKind::King) => "♚",
        }
    }

    // Get the "simple" character to represent this piece (capitalized based on the piece's color)
    pub fn simple_char(&self) -> char {
        match (&self.color, &self.kind) {
            (Sides::White, PieceKind::Pawn ) => 'P',
            (Sides::White, PieceKind::Knight ) => 'N',
            (Sides::White, PieceKind::Bishop) => 'B',
            (Sides::White, PieceKind::Rook) => 'R',
            (Sides::White, PieceKind::Queen) => 'Q',
            (Sides::White, PieceKind::King) => 'K',
            (Sides::Black, PieceKind::Pawn ) => 'p',
            (Sides::Black, PieceKind::Knight) => 'n',
            (Sides::Black, PieceKind::Bishop) => 'b',
            (Sides::Black, PieceKind::Rook) => 'r',
            (Sides::Black, PieceKind::Queen) => 'q',
            (Sides::Black, PieceKind::King) => 'k',
        }
    }
    pub fn gen_all()->[Self;12]{
       [
            Piece::pawn(Sides::White),
            Piece::knight(Sides::White),
            Piece::bishop(Sides::White),
            Piece::rook(Sides::White),
            Piece::queen(Sides::White),
            Piece::king(Sides::White),

            Piece::pawn(Sides::Black),
            Piece::knight(Sides::Black),
            Piece::bishop(Sides::Black),
            Piece::rook(Sides::Black),
            Piece::queen(Sides::Black),
            Piece::king(Sides::Black),
        ]
    }
}



#[repr(u8)]
pub enum Castle{
    NA = 0,
    WK = 1,
    WQ = 2,
    BK = 4,
    BQ = 8,
}


impl BitOr<Castle> for Castle {
    type Output = Castle;

    fn bitor(self, castle: Castle) -> Self::Output {
            self | castle
        }

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

pub fn mask_knight(sq: Squares) -> Bitboard {
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

pub fn mask_king(sq: Squares) -> Bitboard {
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
pub fn mask_bishop(sq: Squares, block: Bitboard) -> Bitboard {
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

pub fn mask_rook(sq: Squares) -> Bitboard {
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


pub fn mask_queen(
    sq:Squares,
    occupancy:&Bitboard,
    b_mask:&Vec<Bitboard>,
    b_attacks:&Vec<Bitboard>,
    r_mask:&Vec<Bitboard>,
    r_attacks:&Vec<Bitboard>
    ) -> Bitboard {
        get_bishop_attacks(sq, occupancy, b_mask , b_attacks) |  get_rook_attacks(sq, occupancy, r_mask , r_attacks) 
}
