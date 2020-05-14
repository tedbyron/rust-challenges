//! given a string containing brackets `[]`, braces `{}`, parentheses `()`, or
//! any combination of the three, return whether or not all pairs are balanced
//! and nested correctly

#[allow(dead_code)]
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => brackets.push(c),
            ']' | '}' | ')' => match brackets.pop() {
                Some(v) => match (v, c) {
                    ('[', ']') | ('{', '}') | ('(', ')') => continue,
                    _ => return false,
                },
                None => return false,
            },
            _ => continue,
        }
    }

    brackets.is_empty()
}
