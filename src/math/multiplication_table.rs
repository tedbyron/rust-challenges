//! given the number of rows and columns in a multiplication table, produce the
//! multiplication table as a two dimensional list

#[allow(dead_code)]
pub fn multiplication_table(rows: u32, cols: u32) -> Vec<Vec<u32>> {
    (1..=rows)
        .map(|i| (1..=cols).map(|j| i * j).collect::<Vec<u32>>())
        .collect()
}
