//! return the sum of numbers starting at 1 and doubling each step 64 times

fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1 << (s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

#[allow(dead_code)]
pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
