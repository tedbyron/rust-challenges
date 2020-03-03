// get all the divisors from 2 to an integer, or else return a string saying the
// integer is prime
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
