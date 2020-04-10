//! return the reverse of a string

#[allow(dead_code)]
pub fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}
