//! write an interpreter for the ministringfuck language which has two
//! operators: `+` increments the memory cell and wraps to 0 at 256, and `.`
//! outputs the ASCII value of the memory cell

#[allow(dead_code)]
pub fn my_first_interpreter(code: &str) -> String {
    code.chars()
        .fold((String::new(), 0), |(mut s, mut cell): (String, u8), c| {
            match c {
                '+' => cell = cell.wrapping_add(1),
                '.' => s.push(char::from(cell)),
                _ => (),
            }
            (s, cell)
        })
        .0
}
