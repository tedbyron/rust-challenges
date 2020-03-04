// given a list of digits, return the smallest number that could be formed from
// these digits, using the digits only once

pub fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}
