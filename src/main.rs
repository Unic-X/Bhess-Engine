mod board;
use board::display;
use board::Squares;
mod piece;

fn main() {

    display(&[Squares::h8]);
    for rank in (1..9).rev() {
        println!("a{rank}, b{rank}, c{rank}, d{rank}, e{rank}, f{rank}, g{rank}, h{rank},  ")
    }
}
