//! given a count of blue and red marbles put in a bag and the number of blue
//! and red marbles removed so far, return the probability that the next marble
//! removed from the bag will be blue

#[allow(dead_code)]
pub fn guess_blue(blue_start: f64, red_start: f64, blue_pulled: f64, red_pulled: f64) -> f64 {
    let blue_remaining = blue_start - blue_pulled;
    let total_remaining = blue_remaining + red_start - red_pulled;

    blue_remaining / total_remaining
}
