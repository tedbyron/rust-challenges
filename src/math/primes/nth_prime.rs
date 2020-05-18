//! given an integer `n`, calculate the `n`-th prime number

use std::iter;

#[allow(
    dead_code,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn nth(n: usize) -> u32 {
    iter::once(2)
        .chain((3..).step_by(2))
        .filter(|&x| {
            (3..=(x as f64).sqrt() as u32)
                .step_by(2)
                .all(|y| x % y != 0)
        })
        .nth(n)
        .unwrap()
}
