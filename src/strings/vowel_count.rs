//! return the count of vowels in a string

#[allow(dead_code)]
pub fn get_count(string: &str) -> usize {
    string.chars().filter(|&c| "aeiou".contains(c)).count()
}

// use regex::Regex;

// pub fn get_count(string: &str) -> usize {
//     Regex::new("[aeiou]").unwrap().find_iter(string).count()
// }
