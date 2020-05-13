//! given start and end integers, return the lyrics of the beer song within the
//! range

fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        n => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle{2} of beer on the wall.\n", n, n - 1, if n == 2 { "" } else { "s" }),
    }
}

#[allow(dead_code)]
pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .map(verse)
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}
