//! given a maximum integer `n`, return the difference between the square of the
//! sum of the first `n` positive integers and the sum of the squares of the
//! first `n` positive integers

#[allow(clippy::missing_const_for_fn)]
fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

const fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

#[allow(dead_code)]
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
