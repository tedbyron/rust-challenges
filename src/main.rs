#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::color_choice::check_choose;

fn main() {
    println!("{:?}", check_choose(6, 4));
    println!("{:?}", check_choose(4, 2));
}
