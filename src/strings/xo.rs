//! return whether a string has an equal number of x and o chars

#[allow(dead_code)]
pub fn xo(string: &'static str) -> bool {
    string.to_lowercase().matches('x').count() == string.to_lowercase().matches('o').count()
}
