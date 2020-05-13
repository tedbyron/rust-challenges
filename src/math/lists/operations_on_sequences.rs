//! given a list of positive integers with a length that is a multiple of four,
//! solve for two numbers `A` and `B` such that `P = (x(1) ** 2 + x(2) ** 2) *
//! (x(3) ** 2 + x(4) ** 2) * ... * (x(n) ** 2 + x(n+1) ** 2)` and `P = A ** 2 +
//! B ** 2`

#[allow(dead_code, clippy::needless_pass_by_value)]
pub fn solve(arr: Vec<i128>) -> (i128, i128) {
    let first_four = vec![
        i128::abs(arr[0] * arr[2] - arr[1] * arr[3]),
        (arr[0] * arr[3] + arr[1] * arr[2]),
    ];

    if arr.len() == 4 {
        (first_four[0], first_four[1])
    } else {
        solve([first_four.as_slice(), &arr[4..]].concat())
    }
}
