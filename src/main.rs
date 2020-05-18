#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod datetime;
mod interpreters;
mod math;
mod strings;

use math::primes::nth_prime::nth;

fn main() {
    println!("{}", nth(0));
    println!("{}", nth(1));
    println!("{}", nth(2));
    println!("{}", nth(5));
}
