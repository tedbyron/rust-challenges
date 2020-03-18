//! calculate whether to guess blue or red given the number of blue and red
//! marbles put in the bag at the start and the number of blue and red marbles
//! removed so far

#[allow(dead_code)]
pub fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f64 {
    f64::from(blue_start - blue_pulled)
        / f64::from(blue_start - blue_pulled + red_start - red_pulled)
}
