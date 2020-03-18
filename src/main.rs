#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::bag_of_marbles::guess_blue;

fn main() {
    println!("{:?}", guess_blue(5, 5, 2, 3));
}
