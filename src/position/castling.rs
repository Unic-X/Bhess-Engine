use std::ops::{BitOr, BitOrAssign, Not};

#[derive(Debug,PartialEq)]
#[repr(u8)]
pub enum Castle{
    NA = 0,
    WK = 1,
    WQ = 2,
    BK = 4,
    BQ = 8,
}

// WQ BK 
// 


impl Castle {
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Castle::NA,
            1 => Castle::WK,
            2 => Castle::WQ,
            4 => Castle::BK,
            8 => Castle::BQ,
            _ => Castle::NA, // Default case for combined bits
        }
    }

    pub fn bits(&self) -> u8 {
        match self {
            &Castle::NA => 0,
            &Castle::WK => 1,
            &Castle::WQ => 2,
            &Castle::BK => 4,
            &Castle::BQ => 8,
        }
    }

    pub fn all() -> u8 {
        (Castle::WK | Castle::BK | Castle::WQ | Castle::BQ) as u8
    }
}

impl Not for Castle{
    type Output = Self;
    fn not(self)->Self::Output {
        Self::from_bits(!self.bits())
    }
}

impl BitOr<Castle> for Castle {
    type Output = Castle;

    fn bitor(self, other: Castle) -> Self::Output {
        Self::from_bits(self as u8 | other as u8)
    }
}

impl BitOrAssign for Castle{
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self::from_bits(self.bits() | rhs.bits())
    }
}

#[derive(Debug)]
pub struct CastleRights(pub Castle);

