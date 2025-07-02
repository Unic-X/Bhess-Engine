use std::ops::{BitOr, BitOrAssign, Not};

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Castle {
    NA = 0, //None
    WK = 1,
    WQ = 2,
    BK = 4,
    BQ = 8,
}

impl Castle {
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Castle::NA,
            1 => Castle::WK,
            2 => Castle::WQ,
            4 => Castle::BK,
            8 => Castle::BQ,
            _ => Castle::NA, // Default case for none bits
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

    pub fn from(pos: &[Self]) -> Self {
        let bits = pos.iter().fold(0u8, |acc, x| acc | x.bits());
        Self::from_bits(bits)
    }

    pub fn all() -> Castle {
        Castle::WK | Castle::BK | Castle::WQ | Castle::BQ
    }

    pub fn none() -> Self {
        Castle::NA
    }
}

impl Not for Castle {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::from_bits(!self.bits())
    }
}

impl BitOr<Castle> for Castle {
    type Output = Castle;

    fn bitor(self, other: Castle) -> Self::Output {
        Self::from_bits(self as u8 | other as u8)
    }
}

impl BitOrAssign for Castle {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self::from_bits(self.bits() | rhs.bits()) // BUG: from bits only works for a single castle pos
    }
}

#[derive(Debug, PartialEq)]
pub struct CastleRights(pub Castle);


impl CastleRights {
    pub fn bits(&self) -> u8 {
        self.0.bits()
    }

    pub fn contains(&self, flag: Castle) -> bool {
        self.bits() & flag.bits() != 0
    }

    pub fn insert(&mut self, flag: Castle) {
        self.0 |= flag;
    }

    pub fn remove(&mut self, flag: Castle) {
        let bits = self.bits() & !flag.bits();
        self.0 = Castle::from_bits(bits);
    }
}