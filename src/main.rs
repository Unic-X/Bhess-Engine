mod board;
use board::display;
use board::Squares;
mod piece;

fn main() {
    let square1 = Squares::e2;
    let square2 = Squares::e3;
    let square3 = Squares::d1;
    display(&[square1,square2,square3]);
    for rank in (1..9).rev() {
        println!("a{rank}, b{rank}, c{rank}, d{rank}, e{rank}, f{rank}, g{rank}, h{rank},  ")
    }
}
