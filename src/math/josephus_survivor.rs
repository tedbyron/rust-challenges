//! given a number of people to be executed `n` and the number of people skipped
//! after every execution `k`, return the solution (last survivor) of the
//! josephus permutation

#[allow(dead_code)]
pub fn josephus_survivor(n: i32, k: i32) -> i32 {
    (1..=n).fold(1, |acc, i| (acc + k) % i) + 1
}
