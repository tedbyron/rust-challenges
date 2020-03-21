#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::pf_interpreter::interpreter;

fn main() {
    println!(
        "{:?}",
        interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)
    );
}
