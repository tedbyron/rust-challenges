#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::optimize_distance::choose_best_sum;

fn main() {
    println!("{:?}", choose_best_sum(163, 3, &vec![50, 55, 56, 57, 58]));
}
