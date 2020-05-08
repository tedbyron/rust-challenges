//! given a string, make its uppercase letters lowercase and its lowercase
//! uppercase and return the result

#[allow(dead_code, clippy::module_name_repetitions)]
pub fn to_alternating_case(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_ascii_uppercase() => c.to_ascii_lowercase(),
            c if c.is_ascii_lowercase() => c.to_ascii_uppercase(),
            _ => c,
        })
        .collect()
}
