//! given a binomial coefficient `m` and the maximum of a fixed set of numbers
//! `n`, return `k` size of subsets in `1..=n` so that every subset is unique
//!
//! binomial coefficient `m = n! / (k! * (n - k)!)`

#[allow(dead_code)]
pub fn check_choose(m: u64, n: u64) -> Option<u64> {
    let mut result = 1;
    for k in 1..=n / 2 {
        result = result * (n - k + 1) / k;
        if result == m {
            return Some(k);
        }
    }
    None
}
