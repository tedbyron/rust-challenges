//! given `m` combinations and `n` number of available options, return `k`
//! number of options in each combination so that every combination is unique
//! binomial coefficient m = n! / (k! * (n - k)!)

#[allow(dead_code, clippy::cast_possible_wrap)]
pub fn check_choose(m: u64, n: u64) -> i64 {
    let mut result = 1;
    for k in 1..=n / 2 {
        result = result * (n - k + 1) / k;
        if result == m {
            return k as i64;
        }
    }
    -1
}
