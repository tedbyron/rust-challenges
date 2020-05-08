#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod interpreters;
mod math;
mod strings;

use math::bin_to_dec::bin_to_decimal;

fn main() {
    assert_eq!(bin_to_decimal("0"), 0);
    assert_eq!(bin_to_decimal("1"), 1);
    assert_eq!(bin_to_decimal("1001001"), 73);
}
