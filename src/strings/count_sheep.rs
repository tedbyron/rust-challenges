//! output a string that counts sheep from 1 to an input number

#[allow(dead_code)]
pub fn count_sheep(n: u32) -> String {
    (1..=n).map(|i| format!("{} sheep...", i)).collect()
}
