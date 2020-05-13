#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod datetime;
mod interpreters;
mod math;
mod strings;

use math::lists::operations_on_sequences::solve;

fn main() {
    let mut a = vec![1, 3, 1, 2, 1, 5, 1, 9];
    dotest(&a);
    a = vec![0, 7, 0, 0];
    dotest(&a);
    a = vec![2, 1, 3, 4];
    dotest(&a);
    a = vec![3, 9, 8, 4, 6, 8, 7, 8, 4, 8, 5, 6, 6, 4, 4, 5];
    dotest(&a);
}

fn calc(arr: &[i128]) -> i128 {
    let mut p: i128 = 1;
    let mut i = 0;
    while i < (arr.len() - 1) {
        p *= arr[i] * arr[i] + arr[i + 1] * arr[i + 1];
        i += 2;
    }
    p
}

fn check(arr: &[i128], res: (i128, i128)) -> bool {
    if res.0 < 0 || res.1 < 0 {
        println!("A and B should be nonnegative integers");
        false
    } else {
        let p = res.0 * res.0 + res.1 * res.1;
        let pp = calc(arr);
        if p == pp {
            true
        } else {
            println!("Incorrect sum of squares");
            false
        }
    }
}

fn dotest(arr: &[i128]) {
    let ans = solve(arr.to_vec());
    let bl: bool = check(arr, ans);
    assert_eq!(bl, true, "Testing array {:?}", arr);
}
