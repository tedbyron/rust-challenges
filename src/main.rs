#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod interpreters;
mod math;
mod strings;

use strings::zachs_lightsabers::how_many_lightsabers_do_you_own;

fn main() {
    assert_eq!(how_many_lightsabers_do_you_own(""), 0);
    assert_eq!(how_many_lightsabers_do_you_own("Adam"), 0);
    assert_eq!(how_many_lightsabers_do_you_own("Zach"), 18);
    assert_eq!(how_many_lightsabers_do_you_own("zach"), 0);
}
