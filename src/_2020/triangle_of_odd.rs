//! given the triangle of consecutive odd numbers:
//!                         1
//!                      3     5
//!                   7     9    11
//!                13    15    17    19
//!             21    23    25    27    29
//! calculate the row sums of this triangle from the row index

#[allow(dead_code)]
pub fn row_sum_odd_numbers(n: i64) -> i64 {
    n.pow(3)
}
