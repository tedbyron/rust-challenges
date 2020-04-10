//! given a string `text`, return a string containing the alphabetical position
//! of each letter in `text`

#[allow(dead_code)]
pub fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                Some((c.to_ascii_lowercase() as u8 - 96).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
