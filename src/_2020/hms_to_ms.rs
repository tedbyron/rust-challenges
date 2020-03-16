//! create a function that takes integer hours, minutes, and seconds, and
//! converts them to total milliseconds

#[allow(dead_code)]
pub const fn past(h: i32, m: i32, s: i32) -> i32 {
    ((h * 60 + m) * 60 + s) * 1000
}
