// write a simple camel case function that works on strings with
// whitespace separating ASCII characters

#[allow(dead_code)]
pub fn camel_case(s: &str) -> String {
    s.split_whitespace()
        .map(|c| [&c[..1].to_uppercase(), &c[1..]].join(""))
        .collect()
}
