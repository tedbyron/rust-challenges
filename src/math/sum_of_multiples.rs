//! given a limit and a list of factors, return the sum of positive integers
//! until the limit that have any of the factors in the list

#[allow(dead_code)]
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors: Vec<u32> = factors.iter().copied().filter(|&n| n != 0).collect();
    (1..limit)
        .filter(|n| factors.iter().any(|m| n % m == 0))
        .sum()
}
