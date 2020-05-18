//! implement a clock that handles times without dates; the clock can be
//! constructed or modified with positive or negative values and two clocks
//! representing the same time should be equal to each other

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

#[allow(dead_code, clippy::missing_const_for_fn)]
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: (hours * 60 + minutes).div_euclid(60).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
