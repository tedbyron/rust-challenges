//! given two strings, return whether or not the two strings have the same first
//! letter as well as the same last letter

#[allow(dead_code)]
pub fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next() && beast.chars().last() == dish.chars().last()
}
