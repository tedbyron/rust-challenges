//! given an integer `n`, return the sum of the numbers in one row of the
//! following triangle, using `n` as the row index
//!                         1
//!                      3     5
//!                   7     9    11
//!                13    15    17    19
//!             21    23    25    27    29

#[allow(dead_code)]
pub fn row_sum_odd_numbers(n: i64) -> i64 {
    n.pow(3)
}
