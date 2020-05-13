//! given an integer, return the prime factors of the integer

use std::iter;

#[allow(dead_code)]
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut candidates = iter::once(2).chain((3..).step_by(2));
    let mut prime_factors = Vec::new();

    while n > 1 {
        let m = candidates.next().unwrap();
        while n % m == 0 {
            prime_factors.push(m);
            n /= m;
        }
    }

    prime_factors
}
