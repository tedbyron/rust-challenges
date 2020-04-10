//! given a positive integer `n`, output a string that counts sheep from `1..=n`

#[allow(dead_code)]
pub fn count_sheep(n: u32) -> String {
    (1..=n).map(|i| format!("{} sheep...", i)).collect()
}
