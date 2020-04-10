//! given a positive integer `n`, return its number of digits

#[allow(dead_code)]
pub fn digits(n: u64) -> usize {
    n.to_string().len()
}
