//! given a string, return its reverse

use unicode_segmentation::UnicodeSegmentation;

#[allow(dead_code)]
pub fn solution(phrase: &str) -> String {
    UnicodeSegmentation::graphemes(phrase, true).rev().collect()
}
