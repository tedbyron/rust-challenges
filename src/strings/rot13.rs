//! given a string, return the result of a ROT13 substitution cipher on the
//! string

#[allow(dead_code)]
pub fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => char::from(c as u8 + 13),
            'N'..='Z' | 'n'..='z' => char::from(c as u8 - 13),
            _ => c,
        })
        .collect()
}
