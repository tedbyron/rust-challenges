//! cut a rectangle into squares and return a list of the squares arranged from
//! largest to smallest, or None if the original rectangle is a square

use std::cmp;

#[allow(dead_code)]
pub fn sq_in_rect(length: i32, width: i32) -> Option<Vec<i32>> {
    if length == width {
        return None;
    }

    let mut x = length;
    let mut y = width;
    let mut squares: Vec<i32> = Vec::new();

    while x > 0 && y > 0 {
        let min = cmp::min(x, y);
        squares.push(min);
        x = cmp::max(x, y) - min;
        y = min;
    }

    Some(squares)
}
