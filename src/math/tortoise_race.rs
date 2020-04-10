//! given two velocities and a distance, return the amount of time it will it
//! take the second velocity to catch up to the first as a list of hours,
//! minutes, and seconds

#[allow(dead_code)]
pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        None
    } else {
        let t = 3600 * g / (v2 - v1);
        Some(vec![t / 3600, t / 60 % 60, t % 60])
    }
}
