//! return the last element in a josephus permutation

#[allow(dead_code)]
pub fn josephus_survivor(n: i32, k: i32) -> i32 {
    (1..=n).fold(1, |acc, i| (acc + k) % i) + 1
}
