//! given `m` total color combinations and `n` number of available colors, return `x`
//! number of colors per combination so that every combination is unique
//! binomial coefficient m = n! / (x! * (n - x)!)

#[allow(dead_code, clippy::cast_possible_wrap)]
pub fn check_choose(m: u64, n: u64) -> i64 {
    let mut result = 1;
    for x in 1..=n / 2 {
        result = result * (n - x + 1) / x;
        if result == m {
            return x as i64;
        }
    }
    -1
}
