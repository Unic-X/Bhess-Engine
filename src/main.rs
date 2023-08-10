mod board;
use board::display;
use board::Squares;
fn main() {
    let square = Squares::e2;
    display(square);
    for rank in (1..9).rev() {
        println!("a{rank}, b{rank}, c{rank}, d{rank}, e{rank}, f{rank}, g{rank}, h{rank},  ")
    }
}
