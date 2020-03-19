//! write a simple camel case function that works on strings with
//! whitespace separated ASCII characters

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
