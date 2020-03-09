#[allow(dead_code)]
pub fn xo(string: &'static str) -> bool {
    string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
}
