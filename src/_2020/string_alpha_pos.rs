// replace every letter in a string with its position in the alphabet

#[allow(dead_code)]
pub fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c.to_ascii_lowercase() as u8 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
