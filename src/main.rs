#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::bf_interpreter::brain_luck;

fn main() {
    println!(
        "{:?}",
        brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9])
    );
}
