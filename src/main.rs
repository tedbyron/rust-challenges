#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::number_of_digits::digits;

fn main() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9_876_543_210), 10);
}
