use std::cmp;

// cut a rectangle of different length and width into squares
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
