//! calculate whether to guess blue or red given the number of blue and red
//! marbles put in the bag at the start and the number of blue and red marbles
//! removed so far

#[allow(dead_code)]
pub fn guess_blue(blue_start: f64, red_start: f64, blue_pulled: f64, red_pulled: f64) -> f64 {
    let blue_left = blue_start - blue_pulled;
    let total_left = blue_left + red_start - red_pulled;

    blue_left / total_left
}
