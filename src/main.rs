#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod interpreters;
mod math;
mod strings;

use math::primes::k_prime_factors::*;

fn main() {
    testing_count_kprimes(
        5,
        1000,
        1100,
        &[1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100],
    );
    testing_count_kprimes(12, 100_000, 100_100, &[]);

    testing(100, 0);
    testing(144, 0);
    testing(138, 1);
    testing(143, 2);
}

fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: &[i32]) {
    assert_eq!(count_kprimes(k, start, nd), exp)
}

fn testing(n: i32, exp: i32) {
    assert_eq!(puzzle(n), exp)
}
