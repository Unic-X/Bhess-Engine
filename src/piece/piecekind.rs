#[derive(Copy, Clone, PartialEq, Eq, Debug,Hash)]
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
