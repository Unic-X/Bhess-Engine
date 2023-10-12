use samaj::piece::{Sides,mask_pawn};
use samaj::board::{Squares, render};

#[test]
fn test_masking() {
    render(68719476736);
    render(mask_pawn(&[Squares::e4], Sides::White,68719476736));
    assert_eq!( mask_pawn(&[Squares::e4], Sides::White,68719476736),8796093022208);
}
