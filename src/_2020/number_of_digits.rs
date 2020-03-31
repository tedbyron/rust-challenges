//! determine the total number of digits in a given integer `n >= 0`

#[allow(dead_code)]
pub fn digits(n: u64) -> usize {
    n.to_string().len()
}
