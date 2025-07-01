pub mod attacks;
pub mod magic;
mod piecekind;

use piecekind::PieceKind;

use crate::position::Sides;

#[derive(PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Sides,
    pub kind: PieceKind,
}

impl Piece {
    pub fn index(&self) -> usize {
        match (&self.color, &self.kind) {
            (Sides::White, PieceKind::Pawn) => 0,
            (Sides::White, PieceKind::Knight) => 1,
            (Sides::White, PieceKind::Bishop) => 2,
            (Sides::White, PieceKind::Rook) => 3,
            (Sides::White, PieceKind::Queen) => 4,
            (Sides::White, PieceKind::King) => 5,

            (Sides::Black, PieceKind::Pawn) => 6,
            (Sides::Black, PieceKind::Knight) => 7,
            (Sides::Black, PieceKind::Bishop) => 8,
            (Sides::Black, PieceKind::Rook) => 9,
            (Sides::Black, PieceKind::Queen) => 10,
            (Sides::Black, PieceKind::King) => 11,
        }
    }

    pub const fn pawn(color: Sides) -> Self {
        Self {
            kind: PieceKind::Pawn,
            color,
        }
    }

    pub const fn knight(color: Sides) -> Self {
        Self {
            kind: PieceKind::Knight,
            color,
        }
    }

    pub const fn bishop(color: Sides) -> Self {
        Self {
            kind: PieceKind::Bishop,
            color,
        }
    }

    pub const fn rook(color: Sides) -> Self {
        Self {
            kind: PieceKind::Rook,
            color,
        }
    }

    pub const fn queen(color: Sides) -> Self {
        Self {
            kind: PieceKind::Queen,
            color,
        }
    }

    pub const fn king(color: Sides) -> Self {
        Self {
            kind: PieceKind::King,
            color,
        }
    }

    // Get the "fancy" character for this piece
    pub fn fancy_char(&self) -> &'static str {
        match (&self.color, &self.kind) {
            (Sides::White, PieceKind::Pawn) => "♙",
            (Sides::White, PieceKind::Knight) => "♘",
            (Sides::White, PieceKind::Bishop) => "♗",
            (Sides::White, PieceKind::Rook) => "♖",
            (Sides::White, PieceKind::Queen) => "♕",
            (Sides::White, PieceKind::King) => "♔",
            (Sides::Black, PieceKind::Pawn) => "♟",
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
            (Sides::White, PieceKind::Pawn) => 'P',
            (Sides::White, PieceKind::Knight) => 'N',
            (Sides::White, PieceKind::Bishop) => 'B',
            (Sides::White, PieceKind::Rook) => 'R',
            (Sides::White, PieceKind::Queen) => 'Q',
            (Sides::White, PieceKind::King) => 'K',
            (Sides::Black, PieceKind::Pawn) => 'p',
            (Sides::Black, PieceKind::Knight) => 'n',
            (Sides::Black, PieceKind::Bishop) => 'b',
            (Sides::Black, PieceKind::Rook) => 'r',
            (Sides::Black, PieceKind::Queen) => 'q',
            (Sides::Black, PieceKind::King) => 'k',
        }
    }
    pub fn gen_all() -> [Self; 12] {
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
