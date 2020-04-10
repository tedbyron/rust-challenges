//! given a string, return its reverse

#[allow(dead_code)]
pub fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}
