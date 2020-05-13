//! given a `DateTime`, return a `DateTime` one gigasecond in the future

use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;

lazy_static! {
    static ref GIGASECOND: Duration = Duration::seconds(1_000_000_000);
}

// Returns a Utc DateTime one billion seconds after start.
#[allow(dead_code)]
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start
        .checked_add_signed(*GIGASECOND)
        .expect("The `DateTime` sum should not overflow.")
}
