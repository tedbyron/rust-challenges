//! given a matrix of integers, return a list of saddle points where a saddle
//! point is a number that is greater than or equal to every element in its row
//! and less than or equal to every element in its column

#[allow(dead_code, clippy::module_name_repetitions)]
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (i, row) in input.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            if input.iter().all(|r| item <= r[j]) && row.iter().all(|&n| item >= n) {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points
}
