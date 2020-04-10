//! given a count of `rows` and `cols`, return a multiplication table with
//! the input `rows` and `cols`

#[allow(dead_code)]
pub fn multiplication_table(rows: u32, cols: u32) -> Vec<Vec<u32>> {
    (1..=rows)
        .map(|i| (1..=cols).map(|j| i * j).collect::<Vec<u32>>())
        .collect()
}
