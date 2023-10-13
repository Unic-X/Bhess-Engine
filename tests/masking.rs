use samaj::piece::{Sides,mask_pawn, mask_knight, mask_king};
use samaj::board::{Squares, render};

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
