//! given an integer, return whether or not it is an armstrong number

#[allow(dead_code, clippy::cast_possible_truncation)]
pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    num_string
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_string.len() as u32))
        .sum::<u32>()
        == num
}
