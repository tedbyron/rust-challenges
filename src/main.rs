#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::missing_sequence::remove_nb;

fn main() {
    println!("{:?}", remove_nb(26));
    println!("{:?}", remove_nb(101));
    println!("{:?}", remove_nb(102));
}
