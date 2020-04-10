//! given an list of vulgar fractions, return the list after the fractions have
//! been given their lowest common denominator

// get the greatest common divisor of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// get the least common multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[allow(dead_code)]
pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let lcd = l.iter().fold(1, |acc, n| lcm(acc, n.1));
    l.into_iter().map(|(n, d)| (n * lcd / d, lcd)).collect()
}
