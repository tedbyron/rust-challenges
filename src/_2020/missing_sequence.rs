//! given a positive integer `n`, find two numbers in the sequence `1..n` whose
//! product is equal to the sum of the sequence without the two numbers

#[allow(dead_code)]
pub fn remove_nb(n: i64) -> Vec<(i64, i64)> {
    let s = n * (n + 1) / 2;

    ((s - n) / (n + 1)..n)
        .filter_map(|i| {
            if (s - i) % (i + 1) == 0 {
                Some((i, (s - i) / (i + 1)))
            } else {
                None
            }
        })
        .collect()
}
