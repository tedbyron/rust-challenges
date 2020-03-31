//! given three integers `a, b, c`, return the largest number obtained after
//! inserting the following operators and brackets: +, *, (); in other words,
//! try every combination of `a, b, c` with `[*+()]` , and return the maximum

#[allow(dead_code)]
pub fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[a + b + c, (a + b) * c, a * (b + c), a * b * c]
        .iter()
        .max()
        .unwrap()
}
