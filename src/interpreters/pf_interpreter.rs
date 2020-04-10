//! create a paintfuck interpreter that accepts paintfuck code as a string,
//! the number of iterrations to be performed before the final state of the data
//! grid is returned, and the width and height of the data grid;
//! -   non-command characters should be ignored
//! -   command characters should be treated as case-sensitive
//! -   all values in the data grid should be initialized to 0
//! -   one iteration is treated as the number of command characters evaluated
//! -   the return value of the interpreter should be a representation of the data
//!     grid as a string with rows separated by a CRLF `\r\n`

#[allow(dead_code, clippy::cast_sign_loss)]
pub fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let code: Vec<char> = code.chars().collect();
    let mut data = vec![vec![false; width]; height];
    let (mut idx, mut iter_count, mut ptr_row, mut ptr_col) = (0, 0, 0, 0);

    // move the instruction pointer to the matching loop bracket
    let walk = |i: &mut usize, d: i8| {
        let mut bracket_count = 1;
        while bracket_count != 0 {
            *i = i.wrapping_add(d as usize);
            bracket_count += match code[*i] {
                '[' => d,
                ']' => -d,
                _ => 0,
            };
        }
    };

    while idx < code.len() && iter_count < iterations {
        iter_count += 1;
        match code[idx] {
            'n' => ptr_row = (ptr_row + height - 1) % height,
            'e' => ptr_col = (ptr_col + 1) % width,
            's' => ptr_row = (ptr_row + 1) % height,
            'w' => ptr_col = (ptr_col + width - 1) % width,
            '*' => data[ptr_row][ptr_col] ^= true,
            '[' => {
                if !data[ptr_row][ptr_col] {
                    walk(&mut idx, 1);
                }
            }
            ']' => {
                if data[ptr_row][ptr_col] {
                    walk(&mut idx, -1);
                }
            }
            _ => iter_count -= 1,
        }
        idx += 1;
    }

    data.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|b| if b { '1' } else { '0' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\r\n")
}

// 1-dimensional data grid implementation
//
// #[allow(dead_code, clippy::cast_sign_loss)]
// pub fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
//     let code: Vec<char> = code.chars().collect();
//     let mut data = vec![false; width * height];
//     let (mut idx, mut iter_count, mut ptr_row, mut ptr_col) = (0, 0, 0, 0);

//     // get the data grid index from a given row and column
//     let get_index = |row: usize, col: usize| -> usize { row * width + col };

//     // move the instruction pointer to the matching loop bracket
//     let walk = |i: &mut usize, d: i8| {
//         let mut bracket_count = 1;
//         while bracket_count != 0 {
//             *i = i.wrapping_add(d as usize);
//             bracket_count += match code[*i] {
//                 '[' => d,
//                 ']' => -d,
//                 _ => 0,
//             };
//         }
//     };

//     while idx < code.len() && iter_count < iterations {
//         iter_count += 1;
//         match code[idx] {
//             'n' => ptr_row = (ptr_row + height - 1) % height,
//             'e' => ptr_col = (ptr_col + 1) % width,
//             's' => ptr_row = (ptr_row + 1) % height,
//             'w' => ptr_col = (ptr_col + width - 1) % width,
//             '*' => data[get_index(ptr_row, ptr_col)] ^= true,
//             '[' => {
//                 if !data[get_index(ptr_row, ptr_col)] {
//                     walk(&mut idx, 1);
//                 }
//             }
//             ']' => {
//                 if data[get_index(ptr_row, ptr_col)] {
//                     walk(&mut idx, -1);
//                 }
//             }
//             _ => iter_count -= 1,
//         }
//         idx += 1;
//     }

//     data.chunks_exact(width)
//         .map(|row| {
//             row.iter()
//                 .map(|&b| if b { '1' } else { '0' })
//                 .collect::<String>()
//         })
//         .collect::<Vec<String>>()
//         .join("\r\n")
// }
