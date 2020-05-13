//! given an integer, return a string; if 3 is a factor add `"Pling"` to the
//! string, if 5 is a factor add `"Plang"` to the string, if 7 is a factor, add
//! `"Plong"` to the string, and if the string is empty return the digits of the
//! original number

#[allow(dead_code)]
pub fn raindrops(n: u32) -> String {
    match [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|&(m, s)| if n % m == 0 { Some(s) } else { None })
        .collect::<String>()
    {
        sounds if !sounds.is_empty() => sounds,
        _ => n.to_string(),
    }
}
