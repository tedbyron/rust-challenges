pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let divs = (2..integer)
        .filter(|i| integer % i == 0)
        .collect::<Vec<u32>>();

    if divs.is_empty() {
        Ok(divs)
    } else {
        Err(format!("{} is prime", integer))
    }
}
