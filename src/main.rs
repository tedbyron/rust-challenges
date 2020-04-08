#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::multiplication_table::multiplication_table;

fn main() {
    assert_eq!(multiplication_table(2, 2), vec![vec![1, 2], vec![2, 4]]);
}
