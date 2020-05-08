//! given a binary number as a string slice, return its decimal representation

#[allow(
    dead_code,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
pub fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap()
}
