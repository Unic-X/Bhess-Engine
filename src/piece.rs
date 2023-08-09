pub enum Color{
    White,
    Black
}


struct Position{
    x:u32,
    y:u32
}

pub struct About{
    value: u32,
    position:Position
}


pub enum Piece {
    Pawn(About),
    Knight(About),
    Rook(About),
    Queen(About),
    King(About),
}


impl Piece{

    
    // add code here
}

fn main() {
    
}
