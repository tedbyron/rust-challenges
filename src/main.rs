#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::range_extraction::range_extraction;

fn main() {
    assert_eq!(
        "-6,-3-1,3-5,7-11,14,15,17-20",
        range_extraction(&[-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20])
    );
    assert_eq!(
        "-3--1,2,10,15,16,18-20",
        range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
    );
}
