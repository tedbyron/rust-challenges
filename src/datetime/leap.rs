//! given a year, return whether or not the year is a leap year

#[allow(dead_code)]
pub const fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
