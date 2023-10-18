use samaj::piece::{Sides,mask_pawn, mask_knight, mask_king, mask_bishop, mask_rook};
use samaj::board::{Squares, render};
use samaj::set_bit;

#[test]
fn test_pawn_masking() {
    render(mask_pawn(&[Squares::a2], Sides::White,0));
    assert_eq!( mask_pawn(&[Squares::e3], Sides::White,68719476736),8796093022208);
}
#[test]
fn test_knight_masking() {
    render(mask_knight(&[Squares::d5],0));
    assert_eq!( mask_knight(&[Squares::e3], 68719476736),8796093022208);
}

#[test]
fn test_king_masking() {
    render(mask_king(&[Squares::h1],0));
    assert_eq!( mask_king(&[Squares::e3], 68719476736),8796093022208);
}


#[test]
fn test_bishop_masking() {
    let mut bitboard = 0;
    set_bit!(&[Squares::f6],mut bitboard);
    println!("{}",bitboard);
    render(!(mask_bishop(Squares::d4,0,bitboard)));
    assert_eq!( mask_bishop(Squares::e3, 0,bitboard),8796093022208);
}


#[test]
fn test_rook_masking() {
    render(mask_rook(Squares::d4,0));
    assert_eq!( mask_rook(Squares::e3, 68719476736),8796093022208);
}
