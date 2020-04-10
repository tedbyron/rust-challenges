//! given two non-negative integers `str1` and `str2`, return the last decimal
//! digit of `a^b`, assuming the input will be valid and `str1` and `str2` can
//! be larger than the result of `u64::max_value`

#[allow(dead_code)]
pub fn last_digit(str1: &str, str2: &str) -> u32 {
    if str2 == "0" {
        return 1;
    }
    let x = str1.chars().last().unwrap().to_digit(10).unwrap();
    let m = str2
        .chars()
        .fold(0, |a, c| (a * 10 + c.to_digit(10).unwrap()) % 4);
    let exp = if m == 0 { 4 } else { m };
    x.pow(exp) % 10
}
