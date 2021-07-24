//! write an interpreter that will take brainfuck code and an input to be
//! consumed by the `,` instruction, and return the interpreted code produced
//! by the `.` instruction

#[allow(dead_code, clippy::cast_sign_loss)]
pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code: Vec<char> = code.chars().collect();
    let mut input = input.into_iter();
    let (mut buffer, mut output) = (vec![0_u8; 3000], Vec::new());
    let (mut idx, mut ptr) = (0, 0);

    // move the instruction pointer to the matching loop bracket
    let walk = |i: &mut usize, d: i8| {
        let mut bracket_count = 1;
        while bracket_count != 0 {
            *i = i.wrapping_add(d as usize);
            bracket_count += match code.get(*i) {
                Some('[') => d,
                Some(']') => -d,
                _ => 0,
            };
        }
    };

    while idx < code.len() {
        match code.get(idx) {
            Some('>') => ptr += 1,
            Some('<') => ptr -= 1,
            Some('+') => buffer[ptr] = buffer[ptr].wrapping_add(1),
            Some('-') => buffer[ptr] = buffer[ptr].wrapping_sub(1),
            Some('.') => output.push(buffer[ptr]),
            Some(',') => buffer[ptr] = input.next().unwrap(),
            Some('[') => {
                if buffer[ptr] == 0 {
                    walk(&mut idx, 1);
                }
            }
            Some(']') => {
                if buffer[ptr] != 0 {
                    walk(&mut idx, -1);
                }
            }
            _ => (),
        }
        idx += 1;
    }

    output
}
