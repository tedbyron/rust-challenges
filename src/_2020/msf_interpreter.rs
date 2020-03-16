//! write an interpreter for the ministringfuck language which has two
//! operators: `+` increments the memory cell and wraps to 0 at 256, and `.`
//! outputs the ASCII value of the memory cell

#[allow(dead_code)]
pub fn my_first_interpreter(code: &str) -> String {
    let mut cell: u8 = 0;
    let mut string = String::new();

    for c in code.chars() {
        match c {
            '+' => cell = cell.wrapping_add(1),
            '.' => string.push(cell as char),
            _ => (),
        }
    }

    string
}
