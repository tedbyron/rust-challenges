//! given a string of whitespace-separated characters, return a camelcase
//! representation of the string

use unicode_segmentation::UnicodeSegmentation;

#[allow(dead_code)]
pub fn camel_case(s: &str) -> String {
    UnicodeSegmentation::unicode_words(s)
        .map(|w| {
            UnicodeSegmentation::graphemes(w, true)
                .enumerate()
                .map(|(i, g)| {
                    if i == 0 {
                        g.to_uppercase()
                    } else {
                        g.to_lowercase()
                    }
                })
                .collect::<String>()
        })
        .collect()
}
