use samaj::piece::{Sides,mask_pawn};
use samaj::board::{Squares, render};

#[test]
fn test_pawn_masking() {
    render(mask_pawn(&[Squares::a2], Sides::White,0));
    assert_eq!( mask_pawn(&[Squares::e3], Sides::White,68719476736),8796093022208);
}
