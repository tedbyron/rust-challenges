#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod interpreters;
mod math;
mod strings;

use math::count_red_beads::count_red_beads;

fn main() {
    assert_eq!(count_red_beads(0), 0);
    assert_eq!(count_red_beads(1), 0);
    assert_eq!(count_red_beads(3), 4);
    assert_eq!(count_red_beads(5), 8);
}
