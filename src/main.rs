#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::rot13::rot13;

fn main() {
    assert_eq!(rot13("test"), "grfg");
    assert_eq!(rot13("Test"), "Grfg");
}
