//! given an amount of time as integer hours, minutes, and seconds, return the
//! amount as milliseconds

#[allow(dead_code)]
pub const fn past(h: i32, m: i32, s: i32) -> i32 {
    ((h * 60 + m) * 60 + s) * 1000
}
