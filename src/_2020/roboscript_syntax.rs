//! assume an esoteric language roboscript exists and has commands `F`, `L`,
//! `R`, digits `0-9`, and parentheses. Apply syntax highlighting to each
//! command, ignoring parentheses

use regex::{Captures, Regex};

#[allow(dead_code)]
pub fn highlight(code: &str) -> String {
    Regex::new(r"F+|L+|R+|\d+")
        .unwrap()
        .replace_all(code, |c: &Captures| match c[0].chars().next().unwrap() {
            'F' => format!("<span style=\"color: pink\">{}</span>", &c[0]),
            'L' => format!("<span style=\"color: red\">{}</span>", &c[0]),
            'R' => format!("<span style=\"color: green\">{}</span>", &c[0]),
            _ => format!("<span style=\"color: orange\">{}</span>", &c[0]),
        })
        .to_string()
}
