#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod datetime;
mod interpreters;
mod math;
mod strings;

use math::primes::nth_prime::nth;

fn main() {
    for i in 0..1000 {
        println!("{}", nth(i));
    }
}
